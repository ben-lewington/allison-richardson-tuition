use maud::{html, Markup, Render, DOCTYPE};

use crate::routes::ROUTING;

fn head(body: impl Render) -> Markup {
    html! {
        (DOCTYPE)
        head {
            title { "French & German Tutor" }
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="static/tw.css" {}
            link rel="stylesheet" href="static/htmx.css" {}
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
        body ."bg-gray-100 font-mono leading-6"
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
        header ."bg-gray-300 min-w-full text-m grid grid-flow-col flex flex-cols" {
            ."flex flex-rows" {
                a href="/" { img ."m-2 h-17 w-17" src="static/logo.svg" {} }
                h1 ."py-10 pl-4" { "MFL Tutoring Services" }
            }
            (ROUTING.nav(Some(route_index)))
        }
    }
}

pub fn footer(route_index: usize) -> Markup {
    html! {
        footer ."bg-gray-300 absolute bottom-0 left-0 flex flex-rows min-w-full text-xs" {
            ."w-[50%]" {
                (ROUTING.map(route_index))
            }
            address ."w-[50%] text-center mt-8 bottom-0" {
                a href="mailto:a@a.com" ."text-blue-400 underline before:no-underline"
                    ."before:content-['📧_']" { "a@a.com" }
                br;
                a href="tel:+447777777777" ."text-blue-400 underline before:no-underline"
                    ."before:content-['📞_']" { "(+44) 7777 777 777" }
            }
        }
    }
}

pub fn standard(main: impl Render, route_index: usize) -> Markup {
    head(body(header(route_index), main, footer(route_index), ""))
}

pub fn headless(main: impl Render, route_index: usize) -> Markup {
    body(header(route_index), main, footer(route_index), "")
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
