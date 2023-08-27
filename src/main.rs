mod common;
// mod middleware;
mod modal;
mod routes;
mod tw;

use axum::routing::get;
use routes::{about, contact, courses, index};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(index))
        .route("/about", get(about))
        .route("/courses", get(courses))
        .route("/contact", get(contact));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
