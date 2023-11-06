mod parser;
mod interpret;

use std::{process::exit, fmt::Display, fs};
use clap::Parser;

#[derive(Debug, Clone)]
enum Language {
    Cpp,
    C,
    Python,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::C => write!(f, "C"),
            Self::Cpp => write!(f, "Cpp"),
            Self::Python => write!(f, "Python"),
        }
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        match value.as_str() {
            "c" => Language::C,
            "cpp" => Language::Cpp,
            "python" => Language::Python,
            _ => {
                log::error!("Unknown language '{value}'");
                exit(1);
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Input <***.psd> file from which serialization/deserialization library will be generated.
    #[arg(short, long)]
    protofile: String,

    // Target language (cpp,c,python).
    #[arg(short, long)]
    language: Language,

    // Output directory where library will be generated.
    #[arg(short, long)]
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    simple_logger::SimpleLogger::new().without_timestamps().init().unwrap();
    log::info!("Protofile: {}", &args.protofile);
    let language = Language::from(args.language.clone());
    log::info!("Language: {}", args.language);
    let content = fs::read_to_string(args.protofile.as_str())
        .or_else(|_| -> Result<String, String> {
            log::error!("Unable to read {}", &args.protofile);
            exit(1);
        }).unwrap();
    let mut tokens = Vec::<parser::SyntaxToken>::default();
    let tokens = parser::parse(content.as_str()).or_else(|_| -> Result<Vec<parser::SyntaxToken>, String> {
        log::error!("parse error");
        exit(1);
    }).unwrap();
}
