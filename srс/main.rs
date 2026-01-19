use rust_search::{Indexer, SearchFilter};
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser)]
#[command(name = "rust-search")]
#[command(about = "⚡ High-performance sled BwTree full-text search engine")]
#[command(version = "0.2.0")]
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
        #[arg(long, default_value = "results.db")]
        db: String,
    },

    /// Search indexed files
    Search {
        query: String,
        #[arg(long)]
        all: bool,
        #[arg(long)]
        ext: Option<String>,
        #[arg(long)]
        min_size: Option<u64>,
        #[arg(long)]
        max_size: Option<u64>,
        #[arg(long)]
        limit: Option<usize>,
        #[arg(long)]
        case_sensitive: bool,
        #[arg(long, default_value = "results.db")]
        db: String,
    },

    /// Show database statistics
    Stats {
        #[arg(long, default_value = "results.db")]
        db: String,
    },

    /// Optimize database
    Vacuum {
        #[arg(long, default_value = "results.db")]
        db: String,
    },

    /// Clear database
    Clear {
        #[arg(long, default_value = "results.db")]
        db: String,
    },

    /// Start web GUI server
    Server {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value = "3000")]
        port: u16,
        #[arg(long, default_value = "results.db")]
        db: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Index { path, db }) => {
            index_directory(&path, &db).await?;
        }

        Some(Commands::Search {
            query,
            all,
            ext,
            min_size,
            max_size,
            limit,
            case_sensitive,
            db,
        }) => {
            let indexer = Indexer::new(&db)?;
            let mut filter = SearchFilter {
                query,
                search_content: all,
                case_sensitive,
                limit: limit.unwrap_or(100),
                min_size: min_size.unwrap_or(0),
                max_size: max_size.unwrap_or(u64::MAX),
                ..Default::default()
            };

            if let Some(ext_str) = ext {
                filter.extensions = ext_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
            }

            let results = indexer.search(&filter)?;

            if results.is_empty() {
                println!("❌ No results found for: {}", filter.query);
            } else {
                println!(
                    "✅ Found {} results ({} showing):\n",
                    results.len(),
                    results.len().min(filter.limit)
                );

                for (idx, result) in results.iter().enumerate() {
                    println!("{}. {}", idx + 1, result.path);
                    println!("   Score: {:.1} | Size: {} KB | ID: {}", 
                        result.score, 
                        result.size / 1024,
                        result.file_id
                    );

                    if let Some(preview) = &result.matched_content {
                        println!("   Preview: {}", preview);
                    }
                    println!();
                }
            }
        }

        Some(Commands::Stats { db }) => {
            let indexer = Indexer::new(&db)?;
            let stats = indexer.get_stats()?;
            println!("📊 Database Statistics:\n{}", 
                serde_json::to_string_pretty(&stats)?);
        }

        Some(Commands::Vacuum { db }) => {
            let indexer = Indexer::new(&db)?;
            indexer.vacuum()?;
        }

        Some(Commands::Clear { db }) => {
            let indexer = Indexer::new(&db)?;
            indexer.clear()?;
        }

        Some(Commands::Server { host, port, db }) => {
            println!("🚀 Starting server on http://{}:{}", host, port);
            start_server(&host, port, &db).await?;
        }

        None => {
            // Legacy support: auto-detect usage pattern
            if cli.args.is_empty() {
                print_help();
                return Ok(());
            }

            // Check for -n flag (search with limit)
            if cli.args.len() >= 2 && cli.args[0] == "-n" {
                let limit: usize = cli.args[1].parse().unwrap_or(100);
                let query = cli.args[2..].join(" ");
                
                if query.is_empty() {
                    eprintln!("❌ Error: -n requires limit and query");
                    return Ok(());
                }

                let indexer = Indexer::new("results.db")?;
                let filter = SearchFilter {
                    query,
                    limit,
                    ..Default::default()
                };
                let results = indexer.search(&filter)?;
                print_results(&results);
                return Ok(());
            }

            // Auto-index if directory
            if let Some(first_arg) = cli.args.first() {
                if std::path::Path::new(first_arg).is_dir() {
                    let path = PathBuf::from(first_arg);
                    index_directory(&path, "results.db").await?;
                    return Ok(());
                }
            }

            // Otherwise search with default settings
            let query = cli.args.join(" ");
            let indexer = Indexer::new("results.db")?;
            let filter = SearchFilter {
                query,
                ..Default::default()
            };
            let results = indexer.search(&filter)?;
            print_results(&results);
        }
    }

    Ok(())
}

async fn index_directory(path: &std::path::Path, db: &str) -> Result<()> {
    use walkdir::WalkDir;

    println!("📚 Indexing directory: {}", path.display());
    
    // Count files first
    let file_count = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .count();

    // Create progress bar
    let pb = ProgressBar::new(file_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} files")
            .unwrap()
            .progress_chars("=>-")
    );

    let indexer = Indexer::new(db)?;
    let mut count = 0u64;

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let file_path = entry.path();
        if let Ok(_file_info) = indexer.create_file_info_public(file_path).await {
            count += 1;
            pb.inc(1);
        }
    }

    pb.finish_with_message(format!("✅ Indexed {} files", count));
    println!();
    
    Ok(())
}

fn print_help() {
    println!("⚡ rust-search v0.2.0 - High-performance full-text search\n");
    println!("USAGE:");
    println!("    rust-search [COMMAND] [OPTIONS]\n");
    println!("COMMANDS:");
    println!("    index <PATH>              Index a directory");
    println!("    search <QUERY>            Search indexed files");
    println!("    stats                     Show database statistics");
    println!("    vacuum                    Optimize database");
    println!("    clear                     Clear database");
    println!("    server                    Start web GUI server\n");
    println!("LEGACY SUPPORT (v0.1):");
    println!("    rust-search <QUERY>       Search by filename");
    println!("    rust-search -n <N> <Q>    Search with limit N");
    println!("    rust-search <PATH>        Auto-index directory\n");
    println!("SEARCH OPTIONS:");
    println!("    --all                     Search file contents (not just names)");
    println!("    --ext <EXT>               Filter by extension (e.g., --ext rs,py)");
    println!("    --min-size <SIZE>         Minimum file size in bytes");
    println!("    --max-size <SIZE>         Maximum file size in bytes");
    println!("    --limit <N>               Maximum results (default: 100)");
    println!("    --case-sensitive          Case-sensitive search\n");
    println!("EXAMPLES:");
    println!("    rust-search index /home   # Index home directory");
    println!("    rust-search main          # Search for 'main' (legacy)");
    println!("    rust-search -n 10 test    # Search 'test' limit 10 (legacy)");
    println!("    rust-search search test   # Search for 'test' (new)");
    println!("    rust-search search --all --ext rs # Full-text search in Rust");
    println!("    rust-search server --port 3000  # Start GUI on port 3000");
}

fn print_results(results: &[rust_search::SearchResult]) {
    if results.is_empty() {
        println!("❌ No results found");
    } else {
        println!("✅ Found {} results:\n", results.len());
        for (idx, result) in results.iter().enumerate() {
            println!("{}. {}", idx + 1, result.path);
            if let Some(preview) = &result.matched_content {
                println!("   {}", preview);
            }
        }
    }
}

async fn start_server(host: &str, port: u16, db: &str) -> Result<()> {
    println!("Soon");
    println!("📝 Note: Web server implementation coming soon!");
    println!("   For now, use CLI: rust-search search <query>");
    Ok(())
}
