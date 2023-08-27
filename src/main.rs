mod common;
// mod middleware;
mod modal;
mod nav;
mod routes;
mod tw;


use axum::{routing::get, Router, http::HeaderMap};
use common::{standard, headless};
use maud::html;
use tower_http::services::ServeDir;

use crate::common::form;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(|| async {
            (
                hx_trigger_response_headers("navEvt", 0),
                standard(
                    html! {
                        section { "bumper with bg-image" }
                        section { "what we offer" }
                        section { "cards with offerings -> nice bg image" }
                        section { "bumper with poncy quote" }
                    },
                    0,
                ),
            )
        }))
        .route("/about", get(|| async {
            (
                hx_trigger_response_headers("navEvt", 1),
                standard(
                    html! {
                        section {
                            "Allison Richardson"
                        }
                    },
                    1,
                ),
            )
        }))
        .route("/courses", get(|| async {
            (
                hx_trigger_response_headers("navEvt", 2),
                standard(
                    html! {
                        section {
                            "Available Courses"
                        }
                    },
                    2,
                ),
            )
        }))
        .route("/contact", get(|headers: HeaderMap| async move {
            (
                hx_trigger_response_headers("navEvt", 3),
                if let Some(_) = headers.get("HX-Request") {
                    headless(
                        html! {
                            section {
                                (form())
                            }
                        },
                        3
                    )
                } else {
                    standard(
                        html! {
                            section {
                                (form())
                            }
                        },
                        3
                    )
                }
            )
        }));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
