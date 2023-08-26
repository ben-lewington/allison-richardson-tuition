use std::fmt::Display;

use crate::common::with_common_content;

use axum::{http::HeaderMap, response::IntoResponse};
use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static ROUTING: Lazy<Route<'static, usize>> = Lazy::new(|| {
    Route::Nested(
        RouteData::new(0, "/", "Home"),
        vec![
            Route::Simple(RouteData::new(1, "/about", "About Us")),
            Route::Nested(
                RouteData::new(2, "/courses", "Available Courses"),
                vec![
                    Route::Simple(RouteData::new(4, "/courses/french", "French")),
                    Route::Simple(RouteData::new(5, "/courses/german", "German")),
                ],
            ),
            Route::Simple(RouteData::new(3, "/contact", "Contact Us")),
        ],
    )
});

pub struct RouteData<'a, Idx> {
    id: Idx,
    route: &'a str,
    label: &'a str,
}

impl<'a, Idx> RouteData<'a, Idx>
where
    Idx: Display,
{
    pub fn render_as(&self, component_type: &str, style: &str) -> Markup {
        html! {
            a href=(self.route)
                .{ (component_type) "-" (self.id) }
                .(style)
            { (self.label) }
        }
    }
}

pub enum Route<'a, Idx> {
    Simple(RouteData<'a, Idx>),
    Nested(RouteData<'a, Idx>, Vec<Route<'a, Idx>>),
}

impl<'a, Idx> RouteData<'a, Idx> {
    pub fn new(id: Idx, route: &'a str, label: &'a str) -> Self {
        RouteData { id, route, label }
    }
}

impl<'a, Idx> Route<'a, Idx>
where
    Idx: Display + Default + Eq + Copy,
{
    fn sitemap_rec(&self, depth: usize) -> Markup {
        let ml = format!("ml-[{}px]", (depth * 10 * 4).saturating_sub(1));
        html! {
            @match self {
                Route::Simple(rd) => {
                    li ."list-none border-l border-black ml-3 last:border-transparent" {
                        (Self::sitemap_span(rd, depth))
                    }
                }
                Route::Nested(rd, rs) => {
                    li ."list-none border-l border-black ml-3 last:border-transparent" {
                        (Self::sitemap_span(rd, depth))
                        ul ."relative list-none"
                            .(ml)
                        {
                           @for r in rs {
                                (r.sitemap_rec(depth + 1))
                            }
                        }
                    }
                },
            }
        }
    }

    fn sitemap_span(inner: &RouteData<'_, Idx>, depth: usize) -> Markup {
        html! {
            span ."relative inline-block pt-2 pr-2"
                .({
                    if depth > 0 {
                        "pl-8 before:absolute before:top-0 before:bottom-[50%] before:left-[-1px]
                        before:w-8 before:content-[''] before:border-black before:border-b
                        before:border-l"
                    } else { "" }
                })
            {
                (inner.render_as(
                    "sitemap",
                    "relative top-[-2px] inline-block p-2 border whitespace-nowrap"
                ))
           }
        }
    }

    pub fn sitemap(&self) -> Markup {
        html! {
            ."flex flex-wrap m-4"
            _ = "on navEvt(value) from body set sitemapPage to 'sitemap-' + value
                then add @disabled to .{sitemapPage}"
            {
                (self.sitemap_rec(0))
            }
        }
    }

    fn nav_icon_item(inner: &RouteData<'_, Idx>, _depth: usize) -> Markup {
        html! {
              a href=(inner.route) ."block px-4 py-2 border mb-2 bg-gray-300 shadow-xl"
                  ."hover:transition hover:ease-in hover:duration-500 hover:bg-gray-500 hover:animate-pulse"
              { (inner.label) }
        }
    }

    fn _nav_list_item(inner: &RouteData<'_, Idx>, depth: usize) -> Markup {
        html! {
            a href=(inner.route) ."whitespace-nowrap leading-6 " .(format!("px-{}", (depth - 1) * 4)) { (inner.label) }
        }
    }

    fn nav_icon_selectable(inner: &RouteData<'_, Idx>, _depth: usize, current: bool) -> Markup {
        html! {
         a href=(inner.route)
             ."whitespace-nowrap shadow-md ease-in transition duration-300 inline-block mx-2 "
             ."mt-8 px-4 py-2 border min-w-fit max-w-sm "
             .({
                 if current {
                     "bg-gray-300 cursor-crosshair animate-pulse"
                 } else {
                     "bg-transparent hover:ease-in hover:transition hover:duration-300
                     hover:bg-gray-400 hover:shadow-lg hover:animate-pulse"
                 }
             })
         {
             (inner.label)
         }
        }
    }

    fn nav_rec(&self, depth: usize, selection: Option<Idx>) -> Markup {
        if depth == 0 {
            let Route::Nested(_, rs) = self else {
                panic!("expected one home page");
            };

            return html! {
                ul ."flex flex-rows text-s"{
                    @for r in rs {
                        @match r {
                            Route::Simple(rd) => {
                                li {
                                    (Self::nav_icon_selectable(rd, 1, rd.id == selection.unwrap_or(Default::default())))
                                }
                            }
                            Route::Nested(rd, rs) => {
                                li ."group relative inline-block bg-transparent"
                                {
                                    (Self::nav_icon_selectable(rd, 1, rd.id == selection.unwrap_or(Default::default())))
                                    ul ."absolute bg-transparent m-2 z-10 text-sm"
                                        ."transition-opacity opacity-0 delay-0 "
                                        ."group-hover:opacity-100 group-hover:delay-300"
                                    {
                                        @for r in rs { (r.nav_rec(1, None)) }
                                    }
                                }
                            }
                        }

                    }
                }
            };
        }
        html! {
            @match self {
                Route::Simple(rd) => {
                    li {
                    (Self::nav_icon_item(rd, depth))
                    }
                }
                Route::Nested(rd, rs) => {
                    li {
                        (Self::nav_icon_item(rd, depth))
                        ul ."list-none"
                        {
                           @for r in rs {
                                (r.nav_rec(depth + 1, None))
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn nav(&self, current_id: Option<Idx>) -> Markup {
        html! {
            (self.nav_rec(0, current_id))
        }
    }
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

pub async fn index() -> impl IntoResponse {
    (
        hx_trigger_response_headers("navEvt", 0),
        with_common_content(
            html! {
                section { "bumper with bg-image" }
                section { "what we offer" }
                section { "cards with offerings -> nice bg image" }
                section { "bumper with poncy quote" }
            },
            0,
        ),
    )
}

pub async fn about() -> (HeaderMap, Markup) {
    (
        hx_trigger_response_headers("navEvt", 1),
        with_common_content(
            html! {
                section {
                    "Allison Richardson"
                }
            },
            1,
        ),
    )
}

pub async fn courses() -> (HeaderMap, Markup) {
    (
        hx_trigger_response_headers("navEvt", 2),
        with_common_content(
            html! {
                section {
                    "Available Courses"
                }
            },
            2,
        ),
    )
}

pub async fn contact() -> (HeaderMap, Markup) {
    (
        hx_trigger_response_headers("navEvt", 3),
        with_common_content(
            html! {
                form {
                    label for="frm_first_name" {
                        "First Name"
                    }
                    input #frm_first_name ."rounded-md border my-2" type="text" required {}
                }
                a href="/" ."inline-block bg-gray-500 p-5 rounded-md" { "<=" }
            },
            3,
        ),
    )
}
