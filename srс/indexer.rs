use sled::{Db, Error};
use std::path::Path;
use std::fs;
use std::io::Write;
use walkdir::WalkDir;
use crate::models::SearchResult;

pub struct Indexer {
    db: Db,
}

impl Indexer {
    pub fn new(path: &str) -> Result<Self, Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub async fn index_dir(&self, path: &Path) -> anyhow::Result<()> {
        println!("Starting indexing: {}", path.display());
        
        // First pass: count total files
        let mut total = 0;
        for _ in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            total += 1;
        }
        
        println!("Found {} files to index\n", total);
        
        // Second pass: index with progress
        let mut count = 0;
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path_str = entry.path().to_string_lossy().to_string();
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    let size = metadata.len();
                    let mtime = metadata.modified()?
                        .duration_since(std::time::UNIX_EPOCH)?
                        .as_secs();
                    
                    let key = format!("file:{}", count);
                    let value = format!("{}|{}|{}", path_str, size, mtime);
                    self.db.insert(key, value.as_bytes())?;
                    
                    count += 1;
                    let percent = (count * 100) / total;
                    print!("\rIndexing: {}% ({}/{})", percent, count, total);
                    let _ = std::io::stdout().flush();
                }
            }
        }
        
        self.db.flush()?;
        println!("\n✓ Indexed {} files successfully!", count);
        Ok(())
    }

    pub fn search(&self, query: &str) -> anyhow::Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        
        for item in self.db.iter() {
            let (key, value) = item?;
            let key_str = String::from_utf8_lossy(&key);
            
            if key_str.starts_with("file:") {
                let value_str = String::from_utf8_lossy(&value);
                let parts: Vec<&str> = value_str.split('|').collect();
                
                if parts.len() >= 1 && parts[0].to_lowercase().contains(&query.to_lowercase()) {
                    let file_id = key_str.strip_prefix("file:").unwrap_or("0").parse().unwrap_or(0);
                    results.push(SearchResult {
                        file_id,
                        path: parts[0].to_string(),
                        score: 1.0,
                    });
                }
            }
        }
        
        Ok(results)
    }
}
