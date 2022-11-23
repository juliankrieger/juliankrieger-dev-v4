use axum::{
    extract::{Extension, Path},
    response::Html,
};

use maud::{html, Markup, PreEscaped};

use tracing::instrument;

use crate::state::State;

use std::sync::Arc;

#[instrument(skip_all)]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Markup {
    let state = state.clone();

    html! {
        ul {
            @for post in &state.posts {
                li {
                    a href=(post.link) {
                         (post.frontmatter.date)" - "(post.frontmatter.title)
                    }
                 }
            }
        }
    }
}

#[instrument(skip_all)]
pub async fn entry(Extension(state): Extension<Arc<State>>, Path(link): Path<String>) -> Markup {
    let state = state.clone();

    if let Some(post) = state.posts.iter().find(|p| p.frontmatter.slug == link) {
        return html! {
            div {
                (PreEscaped(post.body_html.clone()))
            }
        };
    } else {
        return html! {
            div {
                "404"
            }
        };
    }
}
