use maud::{html, Markup, Render};

pub fn _modal_fragment(inner: impl Render) -> Markup {
    html! {
        #modal _="on closeModal add .closing then wait for animationend then remove me" {
            ."modal-underlay" _="on click trigger closeModal" {}
            ."modal-content" {
                (inner)
            }
        }
    }
}
