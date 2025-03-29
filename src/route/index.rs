use crate::util::AppState;
use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;
use serde_json::json;

pub async fn index(State(AppState { engine, .. }): State<AppState>) -> impl IntoResponse {
    RenderHtml("index.tera", engine, json!({}))
}
