use maud::{html, Markup, Render, DOCTYPE};

use crate::routes::ROUTING;

pub fn standard(main: impl Render, route_index: usize) -> Markup {
    head(body(header(), main, footer(route_index), ""))
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
        body ."bg-default font-mono leading-6 min-h-screen"
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

pub fn header() -> Markup {
    let home = ROUTING.home();
    html! {
        header ."bg-chrome min-w-full text-m grid grid-flow-col flex flex-cols content-center" {
            ."flex flex-rows" {
                (
                    home.htmx_anchor::<Markup>(
                        "min-[0px]:hidden md:block",
                        Some(&html! {
                            img ."m-2 h-17 w-17" src="static/logo.svg" {}
                        })
                    )
                )
                (
                    home.htmx_anchor("py-10 pl-4", Some(&"MFL Tutoring Services"))
                )
            }
            ."min-[0px]:hidden md:flex md:justify-self-end md:px-2 mb-4" {
                (ROUTING.nav())
            }
            ."md:hidden group justify-self-end mr-44" {
                (ROUTING.small_nav())
            }
        }
    }
}

pub fn footer(route_index: usize) -> Markup {
    html! {
        footer ."bg-chrome absolute bottom-0 left-0 grid grid-flow-col min-w-full text-xs" {
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
