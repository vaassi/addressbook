use axum::{routing::get_service, Router};
use tower_http::services::ServeFile;

pub fn routes_data() -> Router {
    Router::new().route_service("/data", get_service(ServeFile::new("./data.json")))
}
