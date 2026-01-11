use rust_search::Indexer;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "rust-search")]
#[command(about = "sled BwTree full-text search engine")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Search query or index path
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Index directory
    Index {
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let indexer = Indexer::new("results.db")?;

    // Shorthand search: rust-search -n Lol
    if cli.args.len() >= 2 && cli.args[0] == "-n" {
        let query = cli.args[1..].join(" ");
        let results = indexer.search(&query)?;
        if results.is_empty() {
            println!("No results found for: {}", query);
        } else {
            for r in results.iter().take(100) {
                println!("{}: {}", r.file_id, r.path);
            }
        }
        return Ok(());
    }

    // Auto-index if path provided as first argument
    if !cli.args.is_empty() && std::path::Path::new(&cli.args[0]).is_dir() {
        let path = PathBuf::from(&cli.args[0]);
        indexer.index_dir(&path).await?;
        return Ok(());
    }

    // Search shorthand: rust-search Lol
    if !cli.args.is_empty() {
        let query = cli.args.join(" ");
        let results = indexer.search(&query)?;
        if results.is_empty() {
            println!("No results found for: {}", query);
        } else {
            for r in results.iter().take(100) {
                println!("{}: {}", r.file_id, r.path);
            }
        }
        return Ok(());
    }

    match cli.command {
        Some(Commands::Index { path }) => {
            indexer.index_dir(&path).await?;
        }
        None => {
            println!("rust-search - sled BwTree search engine");
            println!("\nUsage:");
            println!("  rust-search /home                # Auto-index directory");
            println!("  rust-search -n Lol               # Search with -n flag");
            println!("  rust-search Lol                  # Quick search");
            println!("  rust-search index /home          # Explicit index");
        }
    }

    Ok(())
}

