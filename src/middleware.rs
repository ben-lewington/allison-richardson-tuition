use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

pub struct HtmxRequest(pub bool);

#[async_trait]
impl<S> FromRequestParts<S> for HtmxRequest {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        if parts.headers.contains_key("HX-Request") {
            Ok(HtmxRequest(true))
        } else {
            Ok(HtmxRequest(false))
        }
    }
}
