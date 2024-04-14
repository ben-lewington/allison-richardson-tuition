use maud::{html, Markup};

pub fn form_input(
    form_input_style: &str,
    label_style: &str,
    span_style: &str,
    input_style: &str,
    input_name: &str,
    input_type: &str,
) -> Markup {
    html! {
        .(form_input_style) ."col-span-2" {
            label for=(input_name) .(label_style) {
                span .(span_style) {
                    "Subject"
                }
            }
            input name=(input_name) type=(input_type) .(input_style) {}
        }
    }
}

//  Embed<'_, (), (),
//      Container((
//          Embed<'_, Label, LabelAttrs, Embed<'_, Span, &str>>,
//          Embed<'_, Input, InputAttrs, &str>,
//      ))
//  >

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
