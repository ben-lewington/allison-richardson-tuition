mod common;
mod middleware;
mod modal;
mod nav;
mod routes;
mod svg;

use axum::{http::HeaderMap, response::IntoResponse, routing::get, Router};
use common::standard;
use maud::html;
use middleware::HtmxRequest;
use tower_http::services::ServeDir;

use crate::common::form;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
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
                (
                    hx_trigger_response_headers("navEvt", 3),
                    if t {
                        html! {
                            section ."flex flex-auto" {
                                (form())
                            }
                        }
                    } else {
                        standard(
                            html! {
                                section {
                                    (form())
                                }
                            },
                            3,
                        )
                    },
                )
            }),
        );

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

fn handle_fragment(main: maud::Markup, boosted: bool, id: usize) -> impl IntoResponse {
    (
        hx_trigger_response_headers("navEvt", id),
        if boosted { main } else { standard(main, id) },
    )
}
