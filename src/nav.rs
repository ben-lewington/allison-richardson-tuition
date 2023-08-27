use crate::routes::{Route, RouteData};

use std::fmt::Display;

use maud::{html, Markup};

pub const _MARGINS: [&'static str; 2] = ["ml-[39px]", "ml-[79px]"];

impl<'a, Idx> RouteData<'a, Idx>
where
    Idx: Display + Default + Eq + Copy,
{
    fn map_icon_with_current(&self, depth: usize, route_index: Idx) -> Markup {
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
            a href=(self.route)
                ."relative top-[-2px] inline-block p-2 border whitespace-nowrap"
                .({
                    if self.id == route_index {
                        "bg-gray-300 animate-pulse pointer-events-none"
                    } else {
                        "hover:transition hover:ease-in hover:duration-500 hover:bg-gray-500
                        hover:animate-pulse"
                    }
                })
            { (self.label) }
           }
        }
    }

    fn anchor_icon(&self) -> Markup {
        html! {
              a href=(self.route)
                ."block px-4 py-2 border mb-2 bg-gray-300 shadow-xl hover:transition hover:ease-in "
                ."hover:duration-500 hover:bg-gray-500 hover:animate-pulse"
              { (self.label) }
        }
    }

    fn anchor_icon_with_current(&self, route_index: Idx) -> Markup {
        html! {
         a href=(self.route)
             ."whitespace-nowrap shadow-md ease-in transition duration-300 inline-block mx-2 "
             ."mt-8 px-4 py-2 border min-w-fit max-w-sm "
             .({
                 if route_index == self.id {
                     "bg-gray-300 animate-pulse pointer-events-none"
                 } else {
                     "bg-transparent hover:ease-in hover:transition hover:duration-300
                     hover:bg-gray-400 hover:shadow-lg hover:animate-pulse"
                 }
             })
         {
             (self.label)
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
                (self.map_rec(0, route_index))
            }
        }
    }

    fn map_rec(&self, depth: usize, route_index: Idx) -> Markup {
        let ml = format!("ml-[{}px]", (depth * 10 * 4).saturating_sub(1));
        html! {
            @match self {
                Route::Simple(rd) => {
                    li ."list-none border-l border-black ml-3 last:border-transparent" {
                        (rd.map_icon_with_current(depth, route_index))
                    }
                }
                Route::Nested(rd, rs) => {
                    li ."list-none border-l border-black ml-3 last:border-transparent" {
                        (rd.map_icon_with_current(depth, route_index))
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

    pub fn nav(&self, current: Option<Idx>) -> Markup {
        html! {
            nav ."flex flex-rows" {
                (self.nav_rec(0, current))
            }
        }
    }

    fn nav_rec(&self, depth: usize, current: Option<Idx>) -> Markup {
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
                                    (rd.anchor_icon_with_current(current.unwrap_or(Default::default())))
                                }
                            }
                            Route::Nested(rd, rs) => {
                                li ."group relative inline-block bg-transparent"
                                {
                                    (rd.anchor_icon_with_current(current.unwrap_or(Default::default())))
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
                        (rd.anchor_icon())
                    }
                }
                Route::Nested(rd, rs) => {
                    li {
                        (rd.anchor_icon())
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
}
