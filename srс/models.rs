use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file_id: String,
    pub path: String,
    pub score: f32,
    pub size: u64,
    pub matched_content: Option<String>,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub content: Option<String>,
    pub extension: String,
    pub created_at: String,
    pub modified_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct SearchFilter {
    pub query: String,
    pub search_content: bool,
    pub case_sensitive: bool,
    pub extensions: Vec<String>,
    pub min_size: u64,
    pub max_size: u64,
    pub limit: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub total_files: usize,
    pub total_size: String,
    pub indexed_at: String,
    pub extensions: HashMap<String, usize>,
}
