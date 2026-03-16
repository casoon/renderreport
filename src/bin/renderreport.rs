use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};
use renderreport::{Engine, RenderRequest};

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
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
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
