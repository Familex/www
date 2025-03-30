use crate::util::AppState;
use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;
use serde_json::json;

pub async fn index(State(AppState { engine, .. }): State<AppState>) -> impl IntoResponse {
    RenderHtml(
        "home.jinja",
        engine,
        json!({
            "links": [
                {
                    "icon_file": "https://simpleicons.org/icons/github.svg",
                    "link": "https://github.com/Familex",
                    "short_link": "Familex",
                },
                {
                    "icon_file": "https://simpleicons.org/icons/youtube.svg",
                    "link": "https://youtube.com/@Familex",
                    "short_link": "Familex",
                },
                {
                    "icon_file": "https://simpleicons.org/icons/x.svg",
                    "link": "https://x.com/Familex_",
                    "short_link": "Familex_",
                },
            ]
        }),
    )
}
