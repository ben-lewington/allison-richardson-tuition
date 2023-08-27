use maud::{Markup, html};
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

impl<'a, Idx> RouteData<'a, Idx> {
    pub fn new(id: Idx, route: &'a str, label: &'a str) -> Self {
        RouteData { id, route, label }
    }
}

enum LangCode {

}

enum Ltr {
    LeftToRight,
    RightToLeft
}

enum TwPClass {
    Hover,
    Focus,
    FocusWithin,
    FocusVisible,
    Active,
    Visited,
    Target,
    First,
    Last,
    Only,
    Odd,
    Even,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    Empty,
    Disabled,
    Enabled,
    Checked,
    Indeterminate,
    Default,
    Required,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    PlaceholderShown,
    Autofill,
    ReadOnly,
}

impl TwPClass {
    fn as_css(&self) -> &'static str {
        match self {
            TwPClass::Hover => "hover",
            TwPClass::Focus => "focus",
            TwPClass::FocusWithin => "focus-within",
            TwPClass::FocusVisible => todo!(),
            TwPClass::Active => todo!(),
            TwPClass::Visited => todo!(),
            TwPClass::Target => todo!(),
            TwPClass::First => todo!(),
            TwPClass::Last => todo!(),
            TwPClass::Only => todo!(),
            TwPClass::Odd => todo!(),
            TwPClass::Even => todo!(),
            TwPClass::FirstOfType => todo!(),
            TwPClass::LastOfType => todo!(),
            TwPClass::OnlyOfType => todo!(),
            TwPClass::Empty => todo!(),
            TwPClass::Disabled => todo!(),
            TwPClass::Enabled => todo!(),
            TwPClass::Checked => todo!(),
            TwPClass::Indeterminate => todo!(),
            TwPClass::Default => todo!(),
            TwPClass::Required => todo!(),
            TwPClass::Valid => todo!(),
            TwPClass::Invalid => todo!(),
            TwPClass::InRange => todo!(),
            TwPClass::OutOfRange => todo!(),
            TwPClass::PlaceholderShown => todo!(),
            TwPClass::Autofill => todo!(),
            TwPClass::ReadOnly => todo!(),
        }
    }
}

struct PseudoClassStyle<'a> {
    modifier: PseudoClass,
    style: &'a str,
}

impl<'a> maud::Render for PseudoClassStyle<'a> {
    fn render_to(&self, buffer: &mut String) {
        for style_fragment in self.style.split_whitespace() {
            buffer.push_str(&format!("a"));
        }
    }
}

fn li(
    lbl: &'static str,
    before_style: &'static str,
    hover_style: &'static str,
) -> Markup {
    html! {
        li .relative
            ."pl-4 py-1"
            ."before:bottom-[50%]"
            ."before:left-[-1px]"
            ."before:content-['']"
            ."before:border-l"
            ."before:border-b"
            ."before:border-black"
        {
            span .relative
                ."inline-block"
                ."pt-1 pl-8"
                ."before:absolute"
                ."before:w-8"
                ."before:top-0"
                ."before:bottom-[50%]"
                ."before:left-[-1px]"
                ."before:content-['']"
                ."before:border-l"
                ."before:border-b"
                ."before:border-black"
            {
                a href="#"
                    ."relative"
                    ."top-[-2px]"
                    ."inline-block"
                    ."px-2 py-1"
                    ."border-b"
                    ."whitespace-nowrap"
                    ."hover:transition"
                    ."hover:ease-in"
                    ."hover:duration-500"
                    ."hover:bg-gray-500"
                    ."hover:animate-pulse"
                {
                    (lbl)
                }
            }
        }
    }
}

pub fn small_viewport_nav() -> Markup {
    html! {
        ."group" {
            nav ."flex flex-cols block group-hover:block" {
                ul ."list-none" {
                    ."px-2 py-1 border group-hover:hidden w-fit " { "Menu" }
                    li .relative
                        ."pl-4 py-1"
                        ."before:bottom-[50%]"
                        ."before:left-[-1px]"
                        ."before:content-['']"
                        ."before:border-l"
                        ."before:border-b"
                        ."before:border-black"
                    {
                        span .relative
                            ."inline-block"
                            ."pt-1 pl-8"
                            ."before:absolute"
                            ."before:w-8"
                            ."before:top-0"
                            ."before:bottom-[50%]"
                            ."before:left-[-1px]"
                            ."before:content-['']"
                            ."before:border-l"
                            ."before:border-b"
                            ."before:border-black"
                        {
                            a href="#"
                                ."relative"
                                ."top-[-2px]"
                                ."inline-block"
                                ."px-2 py-1"
                                ."border-b"
                                ."whitespace-nowrap"
                                ."hover:transition"
                                ."hover:ease-in"
                                ."hover:duration-500"
                                ."hover:bg-gray-500"
                                ."hover:animate-pulse"
                            {
                                "About"
                            }
                        }
                    }
                    li .relative
                        ."pl-4 py-1"
                        ."before:bottom-[50%]"
                        ."before:left-[-1px]"
                        ."before:content-['']"
                        ."before:border-l"
                        ."before:border-b"
                        ."before:border-black"
                    {
                        span .relative
                            ."inline-block"
                            ."pt-1 pl-8"
                            ."before:absolute"
                            ."before:w-8"
                            ."before:top-0"
                            ."before:bottom-[50%]"
                            ."before:left-[-1px]"
                            ."before:content-['']"
                            ."before:border-l"
                            ."before:border-b"
                            ."before:border-black"
                        {
                            a href="#"
                                ."relative"
                                ."top-[-2px]"
                                ."inline-block"
                                ."px-2 py-1"
                                ."border-b"
                                ."whitespace-nowrap"
                                ."hover:transition"
                                ."hover:ease-in"
                                ."hover:duration-500"
                                ."hover:bg-gray-500"
                                ."hover:animate-pulse"
                            {
                                "Tuition Services"
                            }
                            ul ."list-none"{
                                li .relative
                                    ."pl-4 py-1"
                                    ."before:bottom-[50%]"
                                    ."before:left-[-1px]"
                                    ."before:content-['']"
                                    ."before:border-l"
                                    ."before:border-b"
                                    ."before:border-black"
                                {
                                    span .relative
                                        ."inline-block"
                                        ."pt-1 pl-8"
                                        ."before:absolute"
                                        ."before:w-8"
                                        ."before:top-0"
                                        ."before:bottom-[50%]"
                                        ."before:left-[-1px]"
                                        ."before:content-['']"
                                        ."before:border-l"
                                        ."before:border-b"
                                        ."before:border-black"
                                    {
                                        a href="#"
                                            ."relative"
                                            ."top-[-2px]"
                                            ."inline-block"
                                            ."px-2 py-1"
                                            ."border-b"
                                            ."whitespace-nowrap"
                                            ."hover:transition"
                                            ."hover:ease-in"
                                            ."hover:duration-500"
                                            ."hover:bg-gray-500"
                                            ."hover:animate-pulse"
                                        {
                                            "French"
                                        }
                                    }
                                }
                                li .relative
                                    ."pl-4 py-1"
                                    ."before:bottom-[50%]"
                                    ."before:left-[-1px]"
                                    ."before:content-['']"
                                    ."before:border-l"
                                    ."before:border-b"
                                    ."before:border-black"
                                {
                                    span .relative
                                        ."inline-block"
                                        ."pt-1 pl-8"
                                        ."before:absolute"
                                        ."before:w-8"
                                        ."before:top-0"
                                        ."before:bottom-[50%]"
                                        ."before:left-[-1px]"
                                        ."before:content-['']"
                                        ."before:border-l"
                                        ."before:border-b"
                                        ."before:border-black"
                                    {
                                        a href="#"
                                            ."relative"
                                            ."top-[-2px]"
                                            ."inline-block"
                                            ."px-2 py-1"
                                            ."border-b"
                                            ."whitespace-nowrap"
                                            ."hover:transition"
                                            ."hover:ease-in"
                                            ."hover:duration-500"
                                            ."hover:bg-gray-500"
                                            ."hover:animate-pulse"
                                        {
                                            "German"
                                        }
                                    }
                                }
                            }
                        }
                    }
                    li .relative
                        ."pl-4 py-1"
                        ."before:bottom-[50%]"
                        ."before:left-[-1px]"
                        ."before:content-['']"
                        ."before:border-l"
                        ."before:border-b"
                        ."before:border-black"
                    {
                        span .relative
                            ."inline-block"
                            ."pt-1 pl-8"
                            ."before:absolute"
                            ."before:w-8"
                            ."before:top-0"
                            ."before:bottom-[50%]"
                            ."before:left-[-1px]"
                            ."before:content-['']"
                            ."before:border-l"
                            ."before:border-b"
                            ."before:border-black"
                        {
                            a href="#"
                                ."relative"
                                ."top-[-2px]"
                                ."inline-block"
                                ."px-2 py-1"
                                ."border-b"
                                ."whitespace-nowrap"
                                ."hover:transition"
                                ."hover:ease-in"
                                ."hover:duration-500"
                                ."hover:bg-gray-500"
                                ."hover:animate-pulse"
                            {
                                "Contact"
                            }
                        }
                    }
                }
            }
        }
    }
}
