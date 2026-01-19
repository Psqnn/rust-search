use crate::models::FileInfo;

pub struct SearchEngine;

impl SearchEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn search_filename(&self, file: &FileInfo, query: &str, case_sensitive: bool) -> f32 {
        let file_name = if case_sensitive {
            file.path.clone()
        } else {
            file.path.to_lowercase()
        };

        let search_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        // Exact match
        if file_name == search_query {
            return 100.0;
        }

        // Contains
        if file_name.contains(&search_query) {
            return 75.0;
        }

        // Word match
        if file_name.split(&['/', '\\', '_', '-', '.'][..]).any(|w| {
            if case_sensitive {
                w == query
            } else {
                w.to_lowercase() == search_query
            }
        }) {
            return 50.0;
        }

        0.0
    }

    pub fn search_content(&self, file: &FileInfo, query: &str, case_sensitive: bool) -> f32 {
        let content = match &file.content {
            Some(c) => c,
            None => return self.search_filename(file, query, case_sensitive),
        };

        let search_content = if case_sensitive {
            content.clone()
        } else {
            content.to_lowercase()
        };

        let search_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        // Count occurrences
        let count = search_content.matches(&search_query).count();
        if count == 0 {
            return self.search_filename(file, query, case_sensitive);
        }

        // Score based on frequency
        let score = (count as f32 * 10.0).min(95.0);
        score
    }

    pub fn extract_preview(
        &self,
        file: &FileInfo,
        query: &str,
        case_sensitive: bool,
    ) -> Option<String> {
        let content = file.content.as_ref()?;
        
        let search_content = if case_sensitive {
            content.clone()
        } else {
            content.to_lowercase()
        };

        let search_query = if case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        // Find first occurrence
        if let Some(pos) = search_content.find(&search_query) {
            let start = pos.saturating_sub(20);
            let end = (pos + search_query.len() + 20).min(content.len());
            
            let preview = &content[start..end];
            let preview = preview.replace('\n', " ");
            
            return Some(format!("...{}...", preview.trim()));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_filename() {
        let file = FileInfo {
            id: "test".to_string(),
            path: "/home/main.rs".to_string(),
            size: 100,
            content: None,
            extension: "rs".to_string(),
            created_at: "2024-01-19".to_string(),
            modified_at: "2024-01-19".to_string(),
        };

        let engine = SearchEngine::new();
        assert!(engine.search_filename(&file, "main", false) > 0.0);
        assert!(engine.search_filename(&file, "test", false) == 0.0);
    }

    #[test]
    fn test_search_content() {
        let file = FileInfo {
            id: "test".to_string(),
            path: "/test.rs".to_string(),
            size: 100,
            content: Some("fn main() { println!(\"hello\"); }".to_string()),
            extension: "rs".to_string(),
            created_at: "2024-01-19".to_string(),
            modified_at: "2024-01-19".to_string(),
        };

        let engine = SearchEngine::new();
        assert!(engine.search_content(&file, "fn", false) > 0.0);
    }
}
