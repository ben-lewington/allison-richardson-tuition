use maud::{html, Markup};
use once_cell::sync::Lazy;

pub static ROUTING: Lazy<Route<'static, usize>> = Lazy::new(|| {
    Route::Nested(
        RouteData::new(0, "/", "Home"),
        vec![
            Route::Simple(RouteData::new(1, "/about", "About Us")),
            Route::Nested(
                RouteData::new(2, "/courses", "Tuition Services"),
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
    pub id: Idx,
    pub route: &'a str,
    pub label: &'a str,
}

pub enum Route<'a, Idx> {
    Simple(RouteData<'a, Idx>),
    Nested(RouteData<'a, Idx>, Vec<Route<'a, Idx>>),
}

impl<'a, Idx> Route<'a, Idx> {
    pub fn home(&self) -> &RouteData<'a, Idx> {
        let Self::Nested(h, _) = self else {
            panic!("only one home page currently supported")
        };
        &h
    }
}

impl<'a, Idx> RouteData<'a, Idx> {
    pub fn new(id: Idx, route: &'a str, label: &'a str) -> Self {
        RouteData { id, route, label }
    }
}

fn small_viewport_icon(
    lbl: &'static str,
    li_style: &'static str,
    span_style: &'static str,
    a_style: &'static str,
    include: impl maud::Render,
) -> Markup {
    html! {
        li .(li_style)
            relative
            ."pl-4"
            ."last:border-transparent"
        {
            span .(span_style)
                .relative
                ."inline-block"
                ."pl-3"
                ."border-black"
                ."before:absolute"
                ."before:w-4"
                ."before:top-0"
                ."before:bottom-[0%]"
                ."before:left-[-3px]"
                ."before:content-['']"
                ."before:border-l"
                ."before:border-b"
                ."before:border-black"
            {
                a href="#"
                    .(a_style)
                    .relative
                    .border-b
                    .border-black
                    ."relative"
                    ."inline-block"
                    ."px-2 py-1"
                    ."whitespace-nowrap"
                    ."bg-hover-pulse"
                {
                    (lbl)
                }
            }
            (include)
        }
    }
}
// ."whitespace-nowrap rounded-md shadow-md ease-in transition duration-300 inline-block mx-2 "
// ."mt-8 px-4 py-2 border min-w-fit max-w-sm "
// ul ."absolute bg-transparent m-2 z-10 text-sm"
//     ."transition-opacity opacity-0 delay-0"
//     ."group-hover:opacity-100 group-hover:delay-300"

pub fn small_viewport_nav() -> Markup {
    html! {
        ."flex group mr-12 mt-8" {
            nav ."absolute justify-self-left" {
                ."" {
                    ."p-2 rounded-md border group-hover:animate-pulse w-fit " { (crate::svg::menu()) }
                    ul ."relative z-10"
                        ."bg-transparent"
                        ."transition-opacity"
                        ."opacity-0"
                        ."delay-0"
                        ."left-[-3px]"
                        ."group-hover:opacity-100 group-hover:delay-300"{
                        (small_viewport_icon("About", "", "", "", ""))
                        (small_viewport_icon("Tuition Services", "", "", "", html! {
                            ul ."relative list-none border-l border-black left-[-3px]" {
                                (small_viewport_icon("French", "", "", "", ""))
                                (small_viewport_icon("German", "", "", "", ""))
                            }
                        }))
                        (small_viewport_icon("Contact", "", "", "", ""))
                    }
                }
            }
        }
    }
}
