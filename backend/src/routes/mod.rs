use crate::models::DomTree;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::algorithms::{bfs, dfs};
use crate::parser::parse;
use crate::scraper::fetch_html;

#[derive(Deserialize)]
pub struct ParseRequest {
    pub html: String,
}

#[derive(Deserialize)]
pub struct ScrapeRequest {
    pub url: String,
}

#[derive(Deserialize)]
pub struct SearchRequest {
    pub html: String,
    pub selector: String,
    pub algorithm: String,
    pub top_n: Option<usize>,
}

#[derive(Serialize)]
pub struct ParseResponse {
    pub tree: DomTree,
    pub node_count: usize,
}

#[derive(Serialize)]
pub struct ScrapeResponse {
    pub html: String,
}

#[derive(Serialize)]
pub struct SearchResponse {
    pub found_indices: Vec<usize>,
    pub traversal_log: Vec<usize>,
    pub tree: DomTree,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn handle_parse(Json(payload): Json<ParseRequest>) -> Json<ParseResponse> {
    let tree = parse(&payload.html);
    let node_count = tree.nodes.len();

    Json(ParseResponse { tree, node_count })
}

pub async fn handle_search(
    Json(payload): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, Json<ErrorResponse>> {
    let tree = parse(&payload.html);

    let top_n = payload.top_n.unwrap_or(0);

    let search_result = match payload.algorithm.as_str() {
        "bfs" => bfs(&tree, &payload.selector, top_n),
        "dfs" => dfs(&tree, &payload.selector, top_n),
        _ => {
            return Err(Json(ErrorResponse {
                error: "Algoritma tidak valid! Gunakan 'bfs' atau 'dfs'.".to_string(),
            }));
        }
    };

    Ok(Json(SearchResponse {
        found_indices: search_result.found_indices,
        traversal_log: search_result.traversal_log,
        tree,
    }))
}

pub async fn handle_scrape(
    Json(payload): Json<ScrapeRequest>,
) -> Result<Json<ScrapeResponse>, Json<ErrorResponse>> {
    match fetch_html(&payload.url).await {
        Ok(html) => Ok(Json(ScrapeResponse { html })),
        Err(e) => Err(Json(ErrorResponse { error: e })),
    }
}
