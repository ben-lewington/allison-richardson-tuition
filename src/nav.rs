use crate::routes::{Route, RouteData};

use std::fmt::Display;

use maud::{html, Markup, Render};

pub const _MARGINS: [&'static str; 2] = ["ml-[39px]", "ml-[79px]"];

impl<'a, Idx> RouteData<'a, Idx>
where
    Idx: Display + Default + Eq + Copy,
{
    pub fn htmx_map_icon(&self, depth: usize) -> Markup {
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
                (self.htmx_anchor::<&'a str>("relative top-[-2px] inline-block p-2 border whitespace-nowrap bg-hover-pulse bg-sitemap hover:bg-sitemap-light:", None))
           }
        }
    }

    pub fn htmx_anchor<M: maud::Render>(&self, style: &'a str, inner: Option<&'a M>) -> Markup {
        html! {
            a href=(self.route)
                hx-get=(self.route)
                hx-target="main"
                .(style) {
                (inner.map(|m| m.render()).unwrap_or(self.label.render()))
            }
        }
    }
}

impl<'a, Idx> Route<'a, Idx>
where
    Idx: Display + Default + Eq + Copy,
{
    pub fn map(&self, route_index: Idx) -> Markup {
        html! {
            ."flex flex-wrap m-4"
            {
                ul {
                    (self.map_rec(0, route_index))
                }
            }
        }
    }

    fn map_rec(&self, depth: usize, route_index: Idx) -> Markup {
        let ml = format!("ml-[{}px]", (depth * 10 * 4).saturating_sub(1));
        html! {
            @match self {
                Route::Simple(rd) => {
                    li ."border-l border-black ml-3 last:border-transparent" {
                        (rd.htmx_map_icon(depth))
                    }
                }
                Route::Nested(rd, rs) => {
                    li ."border-l border-black ml-3 last:border-transparent" {
                        (rd.htmx_map_icon(depth))
                        ul ."relative list-none"
                            .(ml)
                        {
                           @for r in rs {
                                (r.map_rec(depth + 1, route_index))
                            }
                        }
                    }
                },
            }
        }
    }

    pub fn nav(&self) -> Markup {
        html! {
            nav {
                (self.nav_rec(0))
            }
        }
    }

    fn nav_rec(&self, depth: usize) -> Markup {
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
                                    (rd.htmx_anchor::<&'a str>("whitespace-nowrap rounded-md shadow-md inline-block mx-2 mt-8 px-4 py-2 border min-w-fit max-w-sm bg-button bg-hover-pulse", None))
                                }
                            }
                            Route::Nested(rd, rs) => {
                                li ."group relative inline-block bg-transparent"
                                {
                                    (rd.htmx_anchor::<&'a str>("whitespace-nowrap rounded-md shadow-md inline-block mx-2 mt-8 px-4 py-2 border min-w-fit max-w-sm bg-button bg-hover-pulse", None))
                                    ul ."absolute bg-transparent m-2 z-10 text-sm"
                                        ."transition-opacity opacity-0 delay-0"
                                        ."group-hover:opacity-100 group-hover:delay-300"
                                    {
                                        @for r in rs { (r.nav_rec(1)) }
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
                        (rd.htmx_anchor::<&'a str>("block px-4 py-2 border mb-2 bg-anchor shadow-xl bg-hover-pulse", None))
                    }
                }
                Route::Nested(rd, rs) => {
                    li {
                        (rd.htmx_anchor::<&'a str>("block px-4 py-2 border mb-2 bg-anchor shadow-xl bg-hover-pulse", None))
                        ul ."list-none"
                        {
                           @for r in rs {
                                (r.nav_rec(depth + 1))
                            }
                        }
                    }
                }
            }
        }
    }
}
