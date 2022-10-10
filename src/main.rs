mod cipher;
use cipher::Hello;
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let key = "°¡! RüST íS CóÓL ¡!°";
    

    view! { cx,
        div {
            h2 { 
                "Real-Time Vignére Cipher" 
            }
            p { input(placeholder="Enter a phrase", bind:value=name) }
            
        }
    }
}

fn main() {
    sycamore::render(|cx| view! {cx, App {} });
}
