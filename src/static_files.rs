use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::IntoResponse,
};
use maud::{DOCTYPE, html, Markup, Render};
use rust_embed::RustEmbed;

use crate::common::{header, footer};

#[derive(RustEmbed)]
#[folder = "static"]
pub(super) struct StaticFiles;

pub(super) fn serve_static(file: String) -> impl IntoResponse {
    StaticFiles::get(&file)
        .map(|content| {
            let mime = mime_guess::from_path(&file).first_or_octet_stream();
            ([(CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        })
        .unwrap_or(StatusCode::NOT_FOUND.into_response())
}

pub(super) enum HyperMedia<D, H = StaticFiles> {
    Fragment(D),
    Document(D, H),
}

impl<D> HyperMedia<D> {
    pub(super) fn new(inner: D, t: bool) -> Self {
        if t {
            Self::Fragment(inner)
        } else {
            Self::Document(inner, StaticFiles)
        }
    }
}

impl<D: Render> Render for HyperMedia<D> {
    fn render(&self) -> Markup {
        html! {
            @match self {
                Self::Fragment(inner) => { (inner) }
                Self::Document(inner, _) => {
                    (DOCTYPE)
                    head {
                        title { "Login Example" }
                        meta charset="UTF-8";
                        meta name="viewport" content="width=device-width, initial-scale=1";
                        @for f in StaticFiles::iter() {
                            @let href = format!("static/{f}");
                            @match f.split(".").last().unwrap_or("") {
                                "css" => {
                                    link href=(href) rel="stylesheet";
                                }
                                "js" => {
                                    script src=(href) defer {}
                                }
                                "png" => {
                                    @if f.contains("favicon") {
                                        link href=(href) rel="icon" type="image/x-icon";
                                    } @else if f.contains("logo") {
                                        link href=(href) rel="preload" as="image";
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    body ."font-mono bg-default font-mono leading-6 min-h-screen relative min-h-full text-base"
                        hx-boost="true" {
                        (header())
                        main {
                            (inner)
                        }
                        (footer())
                    }
                }
            }
        }
    }
}
