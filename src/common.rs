use maud::{html, Markup, Render, DOCTYPE};

use crate::routes::ROUTING;

pub fn with_common_content<T: Render>(main: T, route_index: usize) -> Markup {
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
        body ."bg-gray-100 font-mono leading-6"
            hx-boost="true"
        {
            header ."bg-gray-300 min-w-full text-m grid grid-flow-col flex flex-cols" {
                ."flex flex-rows" {
                    a href="/" { img ."m-2 h-17 w-17" src="static/logo.svg" {} }
                    h1 ."py-10 pl-4" { "MFL Tutoring Services" }
                }
                nav ."flex flex-rows" {
                    (ROUTING.nav(Some(route_index)))
                }
            }
            main {
                (main)
            }
            footer ."bg-gray-300 absolute bottom-0 left-0 flex flex-rows min-w-full text-xs" {
                ."w-[50%]" {
                    (ROUTING.sitemap())
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
}
