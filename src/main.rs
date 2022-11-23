#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, sync::Arc};

use color_eyre::eyre::Result;

use axum::{
    body,
    http::StatusCode,
    routing::{get, get_service},
    Extension, Router,
};

use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use std::io;

mod assets;
mod handlers;
mod posts;
mod state;

use state::State;

#[instrument]
#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt::init();

    info!("Starting up");

    let state = State::init()?;

    let state = Arc::new(state);

    let middleware = tower::ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(Extension(state.clone()))
        .layer(CorsLayer::permissive());

    let app = Router::new()
        .route(
            "/robots.txt",
            get_service(ServeFile::new("./static/robots.txt")).handle_error(
                |err: io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("unhandled internal server error: {}", err),
                    )
                },
            ),
        )
        .route("/", get(handlers::index::index))
        .route("/blog", get(handlers::blog::index))
        .route("/blog/", get(handlers::blog::index))
        .route("/blog/:link", get(handlers::blog::entry))
        .route("/freelancing", get(handlers::freelancing::index))
        .route("/freelancing/", get(handlers::freelancing::index))
        .route("/publications", get(handlers::publications::index))
        .route("/publications/", get(handlers::publications::index))
        .layer(middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Engines on");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
