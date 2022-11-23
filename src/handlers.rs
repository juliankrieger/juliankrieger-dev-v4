use axum::{response::{IntoResponse, Html}, http::StatusCode};

pub mod blog;
pub mod index;
pub mod freelancing;
pub mod publications;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error: {0}")]
    IO(#[from] std::io::Error)
}

pub type Result<T = Html<Vec<u8>>> = color_eyre::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        return match self {
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
        }
    }
}