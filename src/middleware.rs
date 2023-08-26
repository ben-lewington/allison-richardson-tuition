use axum:: {
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, HeaderValue},
};
use maud::Markup;

pub struct HtmxRequest {
    request: bool
}

//
// pub struct HtmxRequestHeaders<'r> {
//     boosted: bool,
//     current_url: Option<&'r str>,
//     history_restore_request: bool,
//     prompt: Option<&'r str>,
//     request: bool,
//     target: Option<&'r str>,
//     trigger_name: Option<&'r str>,
//     trigger: Option<&'r str>,
// }
//
// #[async_trait]
// impl<'r, 'a> FromRequestParts<()> for HtmxRequestHeaders<'a> {
//     type Rejection = Markup;
//
//     async fn from_request_parts(parts: &'r mut Parts, _: &()) -> Result<Self, Self::Rejection>
//     {
//         Ok(HtmxRequestHeaders {
//             boosted: parts.headers.get("HX-Boosted").map(|_| true).unwrap_or(false),
//             current_url: parts.headers.get("HX-Current-URL").map(|url| url.to_str().unwrap()),
//             history_restore_request: parts.headers.get("HX-History-Restore-Request").map(|_| true).unwrap_or(false),
//             prompt: parts.headers.get("HX-Prompt").map(|r| r.to_str().unwrap()),
//             request: parts.headers.get("HX-Request").map(|_| true).unwrap_or(false),
//             target: parts.headers.get("HX-Target").map(|r| r.to_str().unwrap()),
//             trigger_name: parts.headers.get("HX-Trigger-Name").map(|r| r.to_str().unwrap()),
//             trigger: parts.headers.get("HX-Trigger").map(|r| r.to_str().unwrap()),
//         })
//     }
// }
//
//
// request comes in -> routed appropriately -> partial constructed -> if not HX-Request, embed
// partial in full page
//
//
//
//
