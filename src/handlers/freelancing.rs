use axum::{
    extract::{Extension},
    response::Html,
};

use tracing::{instrument};

use crate::state::State;

use std::sync::Arc;

use crate::templates::freelancing as freelancing_html;

use super::Result;

#[instrument(skip_all)]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Result<Html<Vec<u8>>> {
    let state = state.clone();
    let mut buf: Vec<u8> = vec![];

    freelancing_html(&mut buf)?;
    Ok(Html(buf))
}
