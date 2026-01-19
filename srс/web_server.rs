use axum::{
    routing::{get, post},
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::{Indexer, SearchFilter};

#[derive(Clone)]
pub struct AppState {
    pub indexer: Arc<Indexer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub all: Option<bool>,
    pub ext: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub async fn start_server(host: &str, port: u16, db: &str) -> anyhow::Result<()> {
    let indexer = Arc::new(Indexer::new(db)?);
    let state = AppState { indexer };

    let app = Router::new()
        .route("/", get(handler_root))
        .route("/api/search", post(handler_search))
        .route("/api/stats", get(handler_stats))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
    println!("Soon.....");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handler_root() -> &'static str {
    "🌐 rust-search API v0.2.0 - Use POST /api/search or GET /api/stats"
}

async fn handler_search(
    State(state): State<AppState>,
    Json(payload): Json<SearchRequest>,
) -> Json<ApiResponse<Vec<crate::SearchResult>>> {
    let filter = SearchFilter {
        query: payload.query,
        search_content: payload.all.unwrap_or(false),
        limit: payload.limit.unwrap_or(100),
        ..Default::default()
    };

    match state.indexer.search(&filter) {
        Ok(results) => Json(ApiResponse {
            success: true,
            data: Some(results),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

async fn handler_stats(
    State(state): State<AppState>,
) -> Json<ApiResponse<crate::DatabaseStats>> {
    match state.indexer.get_stats() {
        Ok(stats) => Json(ApiResponse {
            success: true,
            data: Some(stats),
            error: None,
        }),
        Err(e) => Json(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}
