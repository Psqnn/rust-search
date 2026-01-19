use sled::Db;
use crate::models::{SearchResult, SearchFilter, FileInfo, DatabaseStats};
use crate::search_engine::SearchEngine;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;
use chrono::Utc;
use std::collections::HashMap;

pub struct Indexer {
    db: Db,
    engine: SearchEngine,
}

impl Indexer {
    pub fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        let engine = SearchEngine::new();
        Ok(Self { db, engine })
    }

    pub async fn index_dir(&self, path: &Path) -> Result<()> {
        println!("📚 Indexing directory: {}", path.display());
        let mut count = 0;

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
        {
            let file_path = entry.path();
            if let Ok(file_info) = self.create_file_info(file_path).await {
                let id = file_info.id.clone();
                let json = serde_json::to_string(&file_info)?;
                self.db.insert(id.as_bytes(), json.as_bytes())?;
                count += 1;

                if count % 1000 == 0 {
                    println!("  ✓ Indexed {} files", count);
                }
            }
        }

        self.db.flush()?;
        println!("✅ Indexing complete! Total: {} files", count);
        Ok(())
    }

    async fn create_file_info(&self, path: &Path) -> Result<FileInfo> {
        let metadata = std::fs::metadata(path)?;
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content = if self.is_text_file(&extension) {
            std::fs::read_to_string(path).ok()
        } else {
            None
        };

        Ok(FileInfo {
            id: format!("{:x}", md5::compute(path.to_string_lossy().as_bytes())),
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            content,
            extension,
            created_at: Utc::now().to_rfc3339(),
            modified_at: Utc::now().to_rfc3339(),
        })
    }

    // Public method for main.rs (used by taskbar progress)
    pub async fn create_file_info_public(&self, path: &Path) -> Result<FileInfo> {
        let metadata = std::fs::metadata(path)?;
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        let content = if self.is_text_file(&extension) {
            std::fs::read_to_string(path).ok()
        } else {
            None
        };

        let file_info = FileInfo {
            id: format!("{:x}", md5::compute(path.to_string_lossy().as_bytes())),
            path: path.to_string_lossy().to_string(),
            size: metadata.len(),
            content,
            extension,
            created_at: Utc::now().to_rfc3339(),
            modified_at: Utc::now().to_rfc3339(),
        };

        // Store in DB
        let id = file_info.id.clone();
        let json = serde_json::to_string(&file_info)?;
        self.db.insert(id.as_bytes(), json.as_bytes())?;

        Ok(file_info)
    }

    fn is_text_file(&self, extension: &str) -> bool {
        matches!(
            extension.to_lowercase().as_str(),
            "rs" | "py" | "js" | "ts" | "go" | "c" | "cpp" | "h" | "hpp" | "java" | "kt"
                | "swift" | "rb" | "php" | "scala" | "sh" | "bash" | "yml" | "yaml" | "json"
                | "toml" | "xml" | "html" | "css" | "sql" | "md" | "txt" | "log"
        )
    }

    pub fn search(&self, filter: &SearchFilter) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();

        for item in self.db.iter() {
            let (_key, value) = item?;
            let file_info: FileInfo = serde_json::from_slice(&value)?;

            // Check filters
            if !filter.extensions.is_empty()
                && !filter.extensions.contains(&file_info.extension)
            {
                continue;
            }

            if file_info.size < filter.min_size || file_info.size > filter.max_size {
                continue;
            }

            // Search
            let score = if filter.search_content {
                self.engine
                    .search_content(&file_info, &filter.query, filter.case_sensitive)
            } else {
                self.engine
                    .search_filename(&file_info, &filter.query, filter.case_sensitive)
            };

            if score > 0.0 {
                let matched_content = self.engine
                    .extract_preview(&file_info, &filter.query, filter.case_sensitive);

                results.push(SearchResult {
                    file_id: file_info.id,
                    path: file_info.path,
                    score,
                    size: file_info.size,
                    matched_content,
                    created_at: file_info.created_at,
                    modified_at: file_info.modified_at,
                });
            }
        }

        // Sort by score
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(filter.limit);

        Ok(results)
    }

    pub fn get_stats(&self) -> Result<DatabaseStats> {
        let mut extensions = HashMap::new();
        let mut total_size = 0u64;
        let mut total_files = 0usize;

        for item in self.db.iter() {
            let (_key, value) = item?;
            let file_info: FileInfo = serde_json::from_slice(&value)?;

            *extensions.entry(file_info.extension).or_insert(0) += 1;
            total_size += file_info.size;
            total_files += 1;
        }

        Ok(DatabaseStats {
            total_files,
            total_size: self.format_size(total_size),
            indexed_at: Utc::now().to_rfc3339(),
            extensions,
        })
    }

    fn format_size(&self, bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;

        while size > 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{:.2}{}", size, UNITS[unit_idx])
    }

    pub fn vacuum(&self) -> Result<()> {
        self.db.flush()?;
        println!("✅ Database optimized");
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.db.clear()?;
        self.db.flush()?;
        println!("✅ Database cleared");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_text_file() {
        let indexer = Indexer::new("test.db").unwrap();
        assert!(indexer.is_text_file("rs"));
        assert!(indexer.is_text_file("py"));
        assert!(!indexer.is_text_file("png"));
    }
}
