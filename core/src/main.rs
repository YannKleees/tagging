use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use tagging_core::{Config, Mode, Result, Tagger, redact};

#[derive(Parser)]
#[command(name = "tagging", version, about = "Detect and redact personal data in text files")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Print detected spans with byte offsets into the original text
    Detect {
        file: PathBuf,
        #[arg(long, value_enum, default_value_t = Format::Json)]
        format: Format,
        #[arg(long)]
        config: Option<PathBuf>,
    },
    /// Print the text with detected spans masked or pseudonymized
    Redact {
        file: PathBuf,
        #[arg(long, value_enum, default_value_t = RedactMode::Mask)]
        mode: RedactMode,
        #[arg(long)]
        config: Option<PathBuf>,
    },
}

#[derive(Clone, Copy, ValueEnum)]
enum Format {
    Json,
    Csv,
}

#[derive(Clone, Copy, ValueEnum)]
enum RedactMode {
    Mask,
    Pseudo,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Detect { file, format, config } => {
            let text = read(&file)?;
            let spans = tagger(config.as_deref())?.detect(&text);
            match format {
                Format::Json => println!("{}", serde_json::to_string_pretty(&spans)?),
                Format::Csv => {
                    println!("tag,start,end");
                    for s in &spans {
                        println!("{},{},{}", s.kind.as_str(), s.start, s.end);
                    }
                }
            }
        }
        Command::Redact { file, mode, config } => {
            let text = read(&file)?;
            let spans = tagger(config.as_deref())?.detect(&text);
            let mode = match mode {
                RedactMode::Mask => Mode::Mask,
                RedactMode::Pseudo => Mode::Pseudo,
            };
            print!("{}", redact(&text, &spans, mode)?);
        }
    }
    Ok(())
}

fn tagger(config: Option<&Path>) -> Result<Tagger> {
    let config = config.map(Config::from_path).transpose()?.unwrap_or_default();
    Tagger::new(&config)
}

fn read(path: &Path) -> Result<String> {
    std::fs::read_to_string(path).map_err(|source| tagging_core::Error::Io {
        path: path.to_path_buf(),
        source,
    })
}
