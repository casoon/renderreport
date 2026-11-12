use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

use anyhow::Context;
use clap::{Parser, Subcommand, ValueEnum};
use renderreport::engine::EngineConfig;
use renderreport::{ComponentCatalog, Engine, LayoutHint, PageFormat, RenderRequest};

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

#[derive(Clone, Copy, PartialEq, ValueEnum)]
enum OutputFormat {
    Pdf,
    Png,
    Svg,
}

impl OutputFormat {
    fn extension(self) -> &'static str {
        match self {
            OutputFormat::Pdf => "pdf",
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Render a report from a JSON definition
    Render {
        /// Path to JSON input file (reads from stdin if omitted)
        input: Option<PathBuf>,

        /// Output path [default: report.pdf, or report.typ with --typ]; for
        /// several PNG/SVG pages it must contain `{n}`, e.g. page-{n}.png
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Write the generated Typst source instead of compiling it to PDF
        #[arg(long, conflicts_with_all = ["format", "pages"])]
        typ: bool,

        /// Output format [default: from the extension of --output, else pdf]
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,

        /// Pages to write with --format png or svg, e.g. `1`, `1-3`, `2,4`
        /// [default: all]
        #[arg(long, value_name = "SPEC")]
        pages: Option<String>,

        /// Resolution of PNG pages in pixels per inch
        #[arg(long, default_value_t = 144, value_parser = clap::value_parser!(u32).range(1..=2400))]
        ppi: u32,

        /// Directory to search for the pack named by the request's `pack_id`
        /// (repeatable; searched before the default `./packs`)
        #[arg(long = "pack-path", value_name = "DIR")]
        pack_paths: Vec<PathBuf>,

        /// JSON (or .csv) data bound into the request's `{{ placeholders }}`,
        /// `for` and `if`
        #[arg(long, value_name = "FILE")]
        data: Option<PathBuf>,
    },

    /// Check a request without rendering it
    Validate {
        /// Path to JSON input file (reads from stdin if omitted)
        input: Option<PathBuf>,

        /// Directory to search for the pack named by the request's `pack_id`
        /// (repeatable; searched before the default `./packs`)
        #[arg(long = "pack-path", value_name = "DIR")]
        pack_paths: Vec<PathBuf>,

        /// JSON (or .csv) data bound into the request's `{{ placeholders }}`,
        /// `for` and `if`
        #[arg(long, value_name = "FILE")]
        data: Option<PathBuf>,
    },

    /// Print the JSON Schema of a request, or of one component's data
    Schema {
        /// Component ID; without it the schema of the whole request
        #[arg(long)]
        component: Option<String>,
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

fn read_input(input: Option<PathBuf>) -> anyhow::Result<String> {
    match input {
        Some(path) => std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read input file {}", path.display())),
        None => {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read request JSON from stdin")?;
            Ok(buf)
        }
    }
}

/// With `--data`, the request template with the data bound into it.
fn apply_data(json: String, data: Option<PathBuf>) -> anyhow::Result<String> {
    let Some(path) = data else {
        return Ok(json);
    };
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read data file {}", path.display()))?;
    let data: serde_json::Value = if path.extension().is_some_and(|e| e == "csv") {
        let rows = renderreport::render::bind::parse_csv(&text)
            .map_err(|e| anyhow::anyhow!("invalid CSV in {}: {e}", path.display()))?;
        serde_json::json!({ "rows": rows })
    } else {
        serde_json::from_str(&text)
            .with_context(|| format!("failed to parse data file {}", path.display()))?
    };
    let template = serde_json::from_str(&json).context("failed to parse request JSON")?;
    let bound = renderreport::render::bind::bind(&template, &data)
        .context("failed to bind data to the request")?;
    Ok(bound.to_string())
}

/// 1-based page numbers from a spec such as `1`, `1-3` or `2,4`.
fn parse_pages(spec: &str) -> anyhow::Result<Vec<usize>> {
    let number = |text: &str| -> anyhow::Result<usize> {
        match text.trim().parse::<usize>() {
            Ok(n) if n >= 1 => Ok(n),
            _ => anyhow::bail!("invalid --pages `{spec}`: `{text}` is not a page number"),
        }
    };
    let mut pages = Vec::new();
    for part in spec.split(',') {
        match part.split_once('-') {
            Some((first, last)) => {
                let (first, last) = (number(first)?, number(last)?);
                if first > last {
                    anyhow::bail!("invalid --pages `{spec}`: `{part}` is a descending range");
                }
                pages.extend(first..=last);
            }
            None => pages.push(number(part)?),
        }
    }
    Ok(pages)
}

/// Writes the selected pages (all without `pages`): a single page to `output`
/// directly, several pages to `output` with `{n}` replaced by the page number.
fn write_pages(
    rendered: Vec<Vec<u8>>,
    pages: Option<Vec<usize>>,
    output: Option<PathBuf>,
    format: OutputFormat,
) -> anyhow::Result<()> {
    let pages = pages.unwrap_or_else(|| (1..=rendered.len()).collect());
    if let Some(&missing) = pages.iter().find(|&&n| n > rendered.len()) {
        anyhow::bail!(
            "page {missing} does not exist, the document has {} page(s)",
            rendered.len()
        );
    }
    let ext = format.extension();
    let output = output.unwrap_or_else(|| {
        PathBuf::from(if pages.len() == 1 {
            format!("report.{ext}")
        } else {
            format!("report-{{n}}.{ext}")
        })
    });
    let template = output.to_string_lossy();
    if pages.len() > 1 && !template.contains("{n}") {
        anyhow::bail!(
            "{} pages selected but the output path `{template}` has no `{{n}}` to number them, \
             e.g. `page-{{n}}.{ext}`; select one page with --pages or add `{{n}}`",
            pages.len()
        );
    }
    for n in pages {
        let path = PathBuf::from(template.replace("{n}", &n.to_string()));
        let bytes = &rendered[n - 1];
        std::fs::write(&path, bytes)
            .with_context(|| format!("failed to write output file {}", path.display()))?;
        eprintln!("Written {} bytes to {}", bytes.len(), path.display());
    }
    Ok(())
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Schema { component } => {
            let schema = match component {
                Some(id) => renderreport::schema::component_schema(&id).with_context(|| {
                    format!("no schema for component `{id}` (see `list-components`)")
                })?,
                None => renderreport::schema::request_schema(),
            };
            println!("{}", serde_json::to_string_pretty(&schema)?);
            return Ok(());
        }
        Commands::Validate {
            input,
            pack_paths,
            data,
        } => {
            let json = apply_data(read_input(input)?, data)?;
            let request: Option<RenderRequest> = serde_json::from_str(&json).ok();

            // Components of the named pack are valid types; a pack that
            // cannot be loaded is reported by `render`, here it only means
            // its component types are unknown.
            let mut config = EngineConfig::default();
            config.pack_paths.splice(0..0, pack_paths);
            let mut engine =
                Engine::with_config(config).context("failed to initialize render engine")?;
            let mut pack_components = Vec::new();
            if let Some(pack_id) = request.as_ref().and_then(|r| r.pack_id.as_ref()) {
                engine
                    .load_pack(&pack_id.0)
                    .with_context(|| format!("failed to load pack {pack_id}"))?;
                if let Some(pack) = engine.get_pack(pack_id) {
                    pack_components = pack.components.keys().cloned().collect();
                }
            }
            let extra: Vec<&str> = pack_components.iter().map(String::as_str).collect();

            let issues = renderreport::validate::validate_request(&json, &extra);
            for issue in &issues {
                eprintln!("{issue}");
            }
            if !issues.is_empty() {
                anyhow::bail!("{} problem(s) found", issues.len());
            }
            eprintln!("Request is valid");
            return Ok(());
        }
        Commands::ListComponents { markdown, category } => {
            list_components(markdown, category.as_deref());
            return Ok(());
        }
        Commands::Render {
            input,
            output,
            typ,
            format,
            pages,
            ppi,
            pack_paths,
            data,
        } => {
            let format = format
                .or_else(|| {
                    let ext = output.as_ref()?.extension()?.to_str()?.to_ascii_lowercase();
                    OutputFormat::value_variants()
                        .iter()
                        .copied()
                        .find(|f| f.extension() == ext)
                })
                .unwrap_or(OutputFormat::Pdf);
            if pages.is_some() && format == OutputFormat::Pdf && !typ {
                anyhow::bail!("--pages needs --format png or svg");
            }
            let pages = pages.as_deref().map(parse_pages).transpose()?;
            let json = apply_data(read_input(input)?, data)?;

            let request: RenderRequest =
                serde_json::from_str(&json).context("failed to parse request JSON")?;

            let mut config = EngineConfig::default();
            config.pack_paths.splice(0..0, pack_paths);
            let mut engine =
                Engine::with_config(config).context("failed to initialize render engine")?;
            if let Some(pack_id) = &request.pack_id {
                engine
                    .load_pack(&pack_id.0)
                    .with_context(|| format!("failed to load pack {pack_id}"))?;
            }
            if format != OutputFormat::Pdf && !typ {
                let page_format = match format {
                    OutputFormat::Png => PageFormat::Png { ppi },
                    _ => PageFormat::Svg,
                };
                let rendered = engine
                    .render_pages(&request, page_format)
                    .with_context(|| format!("failed to render {} pages", format.extension()))?;
                return write_pages(rendered, pages, output, format);
            }
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
