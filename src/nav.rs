use crate::routes::{Route, RouteData};

use std::fmt::Display;

use maud::{html, Markup, Render};

pub const _MARGINS: [&'static str; 2] = ["ml-[39px]", "ml-[79px]"];

impl<'a, Idx> RouteData<'a, Idx>
where
    Idx: Display + Default + Eq + Copy,
{
    pub fn htmx_map_icon<M: maud::Render>(
        &self,
        span_style: &'a str,
        anchor_style: &'a str,
        inner: Option<&'a M>,
    ) -> Markup {
        html! {
            span
                .(span_style)
            {
                (self.htmx_anchor::<&'a str>(anchor_style, None))
            }
            (inner.map(|m| m.render()).unwrap_or(html! {}))
        }
    }

    pub fn htmx_anchor<M: maud::Render>(&self, style: &'a str, inner: Option<&'a M>) -> Markup {
        html! {
            a href=(self.route)
                hx-get=(self.route)
                hx-target="main"
                hx-push-url="true"
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
                ul { (self.map_rec(0, route_index))
                }
            }
        }
    }

    pub fn nav(&self) -> Markup {
        let Route::Nested(_, rs) = self else {
            panic!("expected one home page");
        };
        html! {
            nav
                _="on navEvt:PostSettle(value) from body log 'Evt' + value"
            {
                (Self::nav_rec(rs, 0))
            }
        }
    }

    pub fn small_nav(&self) -> Markup {
        let Route::Nested(_, rs) = self else {
            panic!("expected one home page");
        };
        html! {
            ."flex group mr-12 mt-8" {
                nav ."absolute justify-self-left" {
                    ."p-2 rounded-md border group-hover:animate-pulse w-fit " { (crate::svg::menu()) }
                    (Self::small_nav_rec(rs, 0))
                }
            }
        }
    }

    fn map_rec(&self, depth: usize, route_index: Idx) -> Markup {
        let span_style = format!("{} relative inline-block pt-2 pr-2", if depth > 0 {
            "pl-8 before:absolute before:top-0 before:bottom-[50%] before:left-[-1px]
            before:w-8 before:content-[''] before:border-black before:border-b before:border-l"
        } else {
            ""
        });
        let anchor_style = "relative top-[-2px] inline-block p-2 border whitespace-nowrap bg-hover-pulse bg-sitemap hover:bg-sitemap-light:";
        html! {
            @match self {
                Route::Simple(rd) => {
                    li ."border-l border-black ml-3 last:border-transparent" {
                        (rd.htmx_map_icon::<&'a str>(&span_style, anchor_style, None))
                    }
                }
                Route::Nested(rd, rs) => {
                    li ."border-l border-black ml-3 last:border-transparent" {
                        (rd.htmx_map_icon::<&'a str>(&span_style, anchor_style, None))
                        ul ."relative list-none"
                            .{ "ml-[" ((depth * 10 * 4).saturating_sub(1)) "px]" }
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

    fn nav_rec(rs: &Vec<Route<'a, Idx>>, depth: usize) -> Markup {
        let anchor_style = if depth == 0 {
            "whitespace-nowrap rounded-md shadow-md inline-block mx-2 mt-8 px-4 py-2 border min-w-fit max-w-sm bg-button bg-hover-pulse"
        } else {
            "block px-4 py-2 rounded-md border mb-2 bg-anchor shadow-xl bg-hover-pulse"
        };

        let ul_style = if depth == 0 {
            "flex flex-rows text-s"
        } else if depth == 1 {
            "absolute bg-transparent m-2 z-10 text-sm transition-opacity opacity-0 delay-0 group-hover:opacity-100 group-hover:delay-300"
        } else {
            "list-none"
        };

        html! {
            ul .(ul_style) {
                @for r in rs {
                    @match r {
                        Route::Simple(rd) => {
                            li {
                                (rd.htmx_map_icon::<&'a str>("", anchor_style, None))
                            }
                        }
                        Route::Nested(rd, rs) => {
                            li ."group relative inline-block bg-transparent"
                            {
                                (rd.htmx_map_icon::<&'a str>("", anchor_style, None))
                                (Self::nav_rec(&rs, depth + 1))
                            }
                        }
                    }
                }
            }
        }
    }

    fn small_nav_rec(rs: &Vec<Route<'a, Idx>>, depth: usize) -> Markup {
        let span_style = "relative inline-block pl-3 border-black before:absolute before:w-4 before:top-0 before:bottom-[0%] before:left-[-3px] before:content-[''] before:border-l before:border-b before:border-black";
        let anchor_style = "relative border-b border-black inline-block px-2 whitespace-nowrap bg-hover-pulse";
        if depth == 0 {
            return html! {
                ul ."relative z-10 bg-transparent transition-opacity opacity-0 delay-0 left-[-3px] group-hover:opacity-100 group-hover:delay-300"
                {
                    @for r in rs {
                        @match r {
                            Route::Simple(rd) => {
                                li ."relative pl-4 last:border-transparent" {
                                    (rd.htmx_map_icon::<&'a str>(span_style, anchor_style, None))
                                }
                            },
                            Route::Nested(rd, rs) => {
                                li ."relative pl-4 last:border-transparent" {
                                    (rd.htmx_map_icon::<Markup>(
                                        span_style,
                                        anchor_style,
                                        Some(&html! {
                                            ul ."relative list-none border-l border-black left-[-3px]" {
                                                (Self::small_nav_rec(rs, depth))
                                            }
                                        })
                                    ))
                                }
                            },
                        }
                    }
                }

            };
        }
        html! {
            ul {
                @for r in rs {
                    @match r {
                        Route::Simple(rd) => {
                            li ."relative pl-4 last:border-transparent" {
                                (rd.htmx_map_icon::<&'a str>(span_style, anchor_style, None))
                            }
                        }
                        Route::Nested(rd, rs) => {
                            li ."relative pl-4 last:border-transparent" {
                                (rd.htmx_map_icon::<&'a str>(span_style, anchor_style, None))
                                ul ."relative list-none border-l border-black left-[-3px]"
                                {
                                    (Self::small_nav_rec(rs, depth + 1))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
