use maud::{html, Markup, Render, DOCTYPE};

use crate::routes::ROUTING;

pub fn standard(main: impl Render, route_index: usize) -> Markup {
    head(body(header(), main, footer(), ""))
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
        body ."font-mono bg-default font-mono leading-6 min-h-screen relative min-h-full text-base"
            hx-boost="true"
        {
            (header)
            // ."bg-[url('/static/paris-matt-hardy.jpg')] bg-cover min-w-full min-h-screen" {
                main {
                    (main)
                }
            // }
            (footer)
            (eob_slot)
        }
    }
}

pub fn header() -> Markup {
    let home = ROUTING.home();
    html! {
        header ."bg-chrome min-w-full text-m grid grid-flow-col flex flex-cols content-center min-h-fit" {
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

pub fn footer() -> Markup {
    html! {
        footer ."absolute bg-chrome bottom-0 left-0 right-0 grid grid-flow-col min-w-full text-xs"
        _="on navEvt(value) from body toggle .hidden on me" {
            (ROUTING.map(0))
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

pub fn inner_form() -> Markup {
    let frm_input_style =
        "grid grid-rows grid-rows-auto ml-4 pt-2 border-l border-black last:border-b";
    let label_style = "ml-3";
    let span_style = "relative inline-block pl-5 pb-1 before:content-[''] before:border-b before:border-l before:border-black before:w-3 before:h-3 before:absolute before:left-[-1px]";
    let input_style = "appearance-none accent-gray-500 mb-2 ml-3 pl-2 py-1.5 mr-2 border-b border-blue-500/30 shadow-inner shadow-blue-500/40 hover:border-sky-500";
    html! {
        form ."grid grid-cols w-full mr-8"
            hx-post="/contact/submit"
        {
            .(frm_input_style) ."col-span-2" {
                span ."ml-3 w-fit border-b border-l pl-3 pb-1 mt-3 border-black"
                {
                    "Name"
                    sup ."text-red-500 font-bold" { " *" }
                }
            }
            .(frm_input_style) {
                label for="first_name" .(label_style) {
                    span ."ml-6"
                    {
                        "First Name"
                    }
                }
                input name="first_name" .(input_style) type="text" required {}
            }
            .(frm_input_style) {
                label for="last_name" .(label_style) {
                    span ."ml-6" {
                        "Last Name" }
                }
                input name="last_name" .(input_style) type="text" required {}
            }
            .(frm_input_style) ."col-span-2" {
                label for="email" .(label_style) {
                    span .(span_style) {
                        "Email"
                        sup ."text-red-500 font-bold" { " *" }
                    }
                }
                input name="email" .(input_style) type="email" required
                _="on htmx:validation:halted
                    if my.value != 'foo' log 'not foo'
                else
                    log 'foo'"
                {}
            }
            .(frm_input_style) ."col-span-2" {
                label for="subject" .(label_style) {
                    span .(span_style) {
                        "Subject"
                    }
                }
                input name="subject" .(input_style) {}
            }
            .(frm_input_style) ."col-span-2" {
                label for="message" .(label_style) {
                    span .(span_style) {
                        "Message"
                        sup ."text-red-500 font-bold" { " *" }
                    }
                }
                textarea name="message" .(input_style) rows="5" cols="10" required {}
            }
            .(frm_input_style) ."w-fit" {
                button #frm_submit ."whitespace-nowrap shadow-md inline-block px-8"
                    ."mx-4 mb-4 py-2 border w-fit max-w-sm bg-button bg-hover-pulse"
                { "Submit" }
            }
        }
    }
}

pub fn form() -> Markup {
    html! {
        ."flex flex-cols mt-20 pl-8" {
            ."min-w-[10%]" {
                "Hello"
            }
            (inner_form())
        }
    }
}
