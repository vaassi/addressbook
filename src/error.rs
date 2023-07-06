use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::ParseError;
use ldap3::LdapError;
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("database error")]
    DatabaseErr(#[from] DbErr),
    #[error("ldap error")]
    LdapErr(#[from] LdapError),
    #[error("parse error")]
    ParseErr(#[from] ParseError),
    #[error("contact not found")]
    ContactNotFound,
    #[error("json error")]
    SedeJsonErr(#[from] serde_json::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "ERROR");

        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": self.to_string()})),
        )
            .into_response()
    }
}
