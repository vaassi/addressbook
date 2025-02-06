use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use std::path::PathBuf;
use tokio::fs;
use tower::util::ServiceExt;
use tower_http::services::ServeDir;

pub fn routes_static() -> Router {
    Router::new().nest_service(
        "/",
        get(|req: Request| async move {
            let result = ServeDir::new("./static").oneshot(req).await.unwrap();
            let status = result.status();
            match status {
                StatusCode::NOT_FOUND => {
                    let index_path = PathBuf::from("./static").join("index.html");
                    fs::read_to_string(index_path)
                        .await
                        .map(|index_content| (StatusCode::OK, Html(index_content)).into_response())
                        .unwrap_or_else(|_| {
                            (StatusCode::INTERNAL_SERVER_ERROR, "index.html not found")
                                .into_response()
                        })
                }

                // path was found as a file in the static dir
                _ => result.into_response(),
            }
        }),
    )
}
