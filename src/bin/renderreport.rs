use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

use anyhow::Context;
use clap::{Parser, Subcommand};
use renderreport::{ComponentCatalog, Engine, LayoutHint, RenderRequest};

#[derive(Parser)]
#[command(
    name = "renderreport",
    about = "Data-driven report generation with Typst as embedded render engine",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a report from a JSON definition
    Render {
        /// Path to JSON input file (reads from stdin if omitted)
        input: Option<PathBuf>,

        /// Output path [default: report.pdf, or report.typ with --typ]
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Write the generated Typst source instead of compiling it to PDF
        #[arg(long)]
        typ: bool,
    },

    /// List all registered component types
    ListComponents {
        /// Output as Markdown instead of plain text
        #[arg(long)]
        markdown: bool,

        /// Filter by category (case-insensitive substring match)
        #[arg(long)]
        category: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", error_chain(&e));
        process::exit(1);
    }
}

/// The whole context chain down to the root cause (e.g. the Typst diagnostic),
/// joined with ": ". Library errors already repeat their source in their own
/// message, so a cause that its predecessor contains is skipped.
fn error_chain(e: &anyhow::Error) -> String {
    let mut out = String::new();
    let mut previous = String::new();
    for cause in e.chain() {
        let message = cause.to_string();
        if !previous.contains(&message) {
            if !out.is_empty() {
                out.push_str(": ");
            }
            out.push_str(&message);
        }
        previous = message;
    }
    out
}

fn list_components(markdown: bool, category_filter: Option<&str>) {
    let mut descriptors: Vec<_> = ComponentCatalog::all()
        .filter(|d| {
            category_filter
                .map(|f| {
                    d.category
                        .label()
                        .to_lowercase()
                        .contains(&f.to_lowercase())
                })
                .unwrap_or(true)
        })
        .collect();

    // Sort alphabetically by ID for deterministic output.
    descriptors.sort_by_key(|d| d.id);

    if markdown {
        println!("# Component Catalog\n");
        println!("| ID | Category | Layout Hint | Description |");
        println!("|---|---|---|---|");
        for d in &descriptors {
            let hint = match d.layout_hint {
                LayoutHint::Breakable => "Breakable",
                LayoutHint::KeepTogether => "KeepTogether",
                LayoutHint::AlwaysNewPage => "AlwaysNewPage",
                LayoutHint::KeepWithNext => "KeepWithNext",
            };
            println!(
                "| `{}` | {} | {} | {} |",
                d.id,
                d.category.label(),
                hint,
                d.description
            );
        }
    } else {
        for d in &descriptors {
            let hint = match d.layout_hint {
                LayoutHint::Breakable => "breakable",
                LayoutHint::KeepTogether => "keep-together",
                LayoutHint::AlwaysNewPage => "always-new-page",
                LayoutHint::KeepWithNext => "keep-with-next",
            };
            println!(
                "{:30} {:25} {:18} {}",
                d.id,
                d.category.label(),
                hint,
                d.description
            );
        }
        println!("\n{} component(s)", descriptors.len());
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::ListComponents { markdown, category } => {
            list_components(markdown, category.as_deref());
            return Ok(());
        }
        Commands::Render { input, output, typ } => {
            let json = match input {
                Some(path) => std::fs::read_to_string(&path)
                    .with_context(|| format!("failed to read input file {}", path.display()))?,
                None => {
                    let mut buf = String::new();
                    io::stdin()
                        .read_to_string(&mut buf)
                        .context("failed to read request JSON from stdin")?;
                    buf
                }
            };

            let request: RenderRequest =
                serde_json::from_str(&json).context("failed to parse request JSON")?;

            let engine = Engine::new().context("failed to initialize render engine")?;
            let (bytes, default_output) = if typ {
                let source = engine
                    .render_typ(&request)
                    .context("failed to generate Typst source")?;
                (source.into_bytes(), "report.typ")
            } else {
                let pdf = engine
                    .render_pdf(&request)
                    .context("failed to render PDF")?;
                (pdf, "report.pdf")
            };

            let output = output.unwrap_or_else(|| PathBuf::from(default_output));
            std::fs::write(&output, &bytes)
                .with_context(|| format!("failed to write output file {}", output.display()))?;
            eprintln!("Written {} bytes to {}", bytes.len(), output.display());
        }
    }

    Ok(())
}
