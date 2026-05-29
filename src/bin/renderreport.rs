use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

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

        /// Output PDF path
        #[arg(short, long, default_value = "report.pdf")]
        output: PathBuf,
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
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn list_components(markdown: bool, category_filter: Option<&str>) {
    let mut descriptors: Vec<_> = ComponentCatalog::all()
        .filter(|d| {
            category_filter
                .map(|f| d.category.label().to_lowercase().contains(&f.to_lowercase()))
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
            println!("{:30} {:25} {:18} {}", d.id, d.category.label(), hint, d.description);
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
        Commands::Render { input, output } => {
            let json = match input {
                Some(path) => std::fs::read_to_string(&path)?,
                None => {
                    let mut buf = String::new();
                    io::stdin().read_to_string(&mut buf)?;
                    buf
                }
            };

            let request: RenderRequest = serde_json::from_str(&json)?;

            let engine = Engine::new()?;
            let pdf = engine.render_pdf(&request)?;

            std::fs::write(&output, &pdf)?;
            eprintln!("Written {} bytes to {}", pdf.len(), output.display());
        }
    }

    Ok(())
}
