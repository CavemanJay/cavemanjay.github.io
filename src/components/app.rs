use std::process::Command;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;
use yew_router::{HashRouter, Switch};

use crate::{switch, Route};

#[function_component]
pub fn App() -> Html {
    #[cfg(debug_assertions)]
    {
        let window = window().expect_throw("Huh?");
        window.document().unwrap().set_title("/optsec - dev");
    }
    let onchange = move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let val = input.value();
        let _ = Command::new(val).spawn().unwrap().wait().unwrap();
    };

    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
            <footer class="flex pb-2">
                <div class="flex-1 text-left pl-3">
                    <label class="block">{"Potential vulnerability"}</label>
                    <input {onchange} />
                </div>
                <div class="flex-grow-[2] text-center">
                    <span>{"Created using: "}</span>
                    <a class="link" target="_blank" href="https://www.rust-lang.org/">
                        <img class="w-8 inline" src="/static/rustacean-flat-gesture.svg" type="image/svg+xml" />
                    </a>
                    <span>{" + "}</span>
                    <a class="link" target="_blank" href="https://yew.rs/">
                        {"yew"}
                    </a>
                    <span>{" + "}</span>
                    <a class="link" target="_blank" href="https://tailwindcss.com/">
                        {"tailwindcss"}
                    </a>
                </div>
                <div class="flex-1 text-right pr-3">
                    <a class="link" target="_blank" href="https://github.com/cavemanjay/cavemanjay.github.io">
                        {"Source"}
                    </a>
                </div>
            </footer>
        </HashRouter>
    }
}
