use maud::{html, Markup, Render, DOCTYPE};

use crate::routes::{ROUTING, small_viewport_nav};

pub fn standard(main: impl Render, route_index: usize) -> Markup {
    head(body(header(route_index), main, footer(route_index), ""))
}

pub fn headless(main: impl Render, route_index: usize) -> Markup {
    body(header(route_index), main, footer(route_index), "")
}

fn head(body: impl Render) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "French & German Tutor" }
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="static/tw.css" {}
            link rel="icon" type="image/x-icon" href="static/favicon.png" {}
            link rel="preload" as="image" href="static/logo.svg" {}
            script src="static/htmx.js" {}
            script src="static/hyscript.js" {}
        }
        (body)
    }
}

fn body(
    header: impl Render,
    main: impl Render,
    footer: impl Render,
    eob_slot: impl Render,
) -> Markup {
    html! {
        body ."bg-gray-100 font-mono leading-6 min-h-screen"
            hx-boost="true"
        {
            (header)
            main {
                (main)
            }
            (footer)
            (eob_slot)
        }
    }
}

pub fn header(route_index: usize) -> Markup {
    html! {
        header ."bg-gray-300 min-w-full text-m grid grid-flow-col flex flex-cols content-center" {
            ."flex flex-rows" {
                a href="/" ."min-[0px]:hidden md:block" { img ."m-2 h-17 w-17" src="static/logo.svg" {} }
                a href="/" ."py-10 pl-4" { "MFL Tutoring Services" }
            }
            ."min-[0px]:hidden md:flex md:justify-self-end md:px-2" {
                (ROUTING.nav(route_index))
            }
            ."md:hidden group justify-self-end mr-44" {
                (small_viewport_nav())
            }
        }
    }
}

pub fn footer(route_index: usize) -> Markup {
    html! {
        footer ."bg-gray-300 absolute bottom-0 left-0 grid grid-flow-col min-w-full text-xs" {
            (ROUTING.map(route_index))
            ."justify-self-end self-end p-4" {
                address ."min-w-full" {
                    ul ."list-none" {
                        li ."py-2" {
                            a href="mailto:a@a.com" ."text-blue-600 underline before:no-underline"
                                ."before:content-['📧_']" { "a@a.com" }
                        }
                        li ."py-2" {
                            a href="tel:+447777777777" ."text-blue-600 underline before:no-underline"
                                ."before:content-['📞_']" { "(+44) 7777 777 777" }
                        }
                    }
                }
            }
        }
    }
}

pub fn form() -> Markup {
    html! {
        form {
            ul {
                li {
                    label for="frm_first_name" {
                        "First Name"
                    }
                    input #frm_last_name ."rounded-md border my-2" type="text" required {}
                }
                li {
                    label for="frm_last_name" {
                        "Last Name"
                    }
                    input #frm_last_name ."rounded-md border my-2" type="text" required {}
                }
            }
        }
    }
}
