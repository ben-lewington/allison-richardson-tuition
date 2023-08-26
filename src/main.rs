mod common;
// mod middleware;
mod routes;
mod tw;

use routes::{about, contact, courses, index};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .route("/", axum::routing::get(index))
        .route("/about", axum::routing::get(about))
        .route("/courses", axum::routing::get(courses))
        .route("/contact", axum::routing::get(contact));

    axum::Server::bind(&"0.0.0.0:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
