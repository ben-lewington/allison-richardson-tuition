mod common;
mod form;
mod middleware;
mod modal;
mod nav;
mod routes;
mod static_files;
mod svg;

use common::{form, standard};
use middleware::HtmxRequest;
use static_files::{serve_static, HyperMedia};

use axum::{
    extract::Path,
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup, Render};
use rust_embed::RustEmbed;

#[derive(Debug, serde::Deserialize)]
pub struct EmailForm {
    first_name: String,
    last_name: String,
    email: String,
    subject: Option<String>,
    message: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/static/*file",
            get(|Path(file): Path<String>| async move { serve_static(file) }),
        )
        .route(
            "/",
            get(|HtmxRequest(t): HtmxRequest| async move {
                handle_fragment(
                    html! {
                        section {
                            "Home Page"
                        }
                    },
                    t,
                    0,
                )
            }),
        )
        .route(
            "/about",
            get(|HtmxRequest(t): HtmxRequest| async move {
                handle_fragment(
                    html! {
                        section {
                            "Allison Richardson"
                        }
                    },
                    t,
                    1,
                )
            }),
        )
        .route(
            "/courses",
            get(|HtmxRequest(t): HtmxRequest| async move {
                handle_fragment(
                    html! {
                        section {
                            "Available Courses"
                        }
                    },
                    t,
                    2,
                )
            }),
        )
        .route(
            "/contact",
            get(|HtmxRequest(t): HtmxRequest| async move {
                let section = html! {
                    section ."overflow-hidden place-content-center" {
                        (form())
                    }
                };

                (
                    hx_trigger_response_headers("navEvt", 3),
                    if t { section } else { standard(section, 3) },
                )
            }),
        )
        .route("/contact/submit", post(handle_form));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_form(HtmxRequest(t): HtmxRequest, Form(f): Form<EmailForm>) -> Markup {
    println!("{f:?}");
    html! { (common::inner_form()) }
}

fn hx_trigger_response_headers<'a>(event_name: &'a str, payload: usize) -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(
        "HX-Trigger",
        format!("{{\"{event_name}\": {payload}}}").parse().unwrap(),
    );
    h.insert(
        "HX-Trigger-After-Settle",
        format!("{{\"{event_name}:PostSettle\": {payload}}}")
            .parse()
            .unwrap(),
    );
    h.insert(
        "HX-Trigger-After-Swap",
        format!("{{\"{event_name}:PostSwap\": {payload}}}")
            .parse()
            .unwrap(),
    );
    h
}

fn handle_fragment(main: maud::Markup, boosted: bool, id: usize) -> impl IntoResponse {
    (
        hx_trigger_response_headers("navEvt", id),
        HyperMedia::new(main, boosted).render(),
    )
}
