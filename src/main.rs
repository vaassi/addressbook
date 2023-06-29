use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use axum::Router;
use sea_orm::DbConn;
use tower_http::cors::CorsLayer;

use crate::db::establish_connection;
use crate::error::Result;
use crate::web::{routes_contact, routes_department, routes_static, routes_sync};

mod db;
mod error;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    // get env vars
    dotenvy::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    // get db conn and apply migrations
    let conn = establish_connection().await?;

    // create app state
    let state = AppState { conn };

    let routes_api = Router::new()
        .merge(routes_department(state.clone()))
        .merge(routes_contact(state.clone()))
        .merge(routes_sync(state));

    let routes = Router::new()
        .nest("/api", routes_api)
        .layer(CorsLayer::permissive())
        .fallback_service(routes_static());

    let addr = SocketAddr::from_str(&server_url).unwrap();

    println!("Listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub conn: DbConn,
}
