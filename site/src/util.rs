use std::io;

use askama::Template;
use axum::{
    body::{self, Full},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use thiserror::Error;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => Error::from(err).into_response(),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error")]
    Io(#[from] io::Error),

    #[error("Failed to render template")]
    Template(#[from] askama::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body::boxed(Full::from(self.to_string())))
            .unwrap()
    }
}

