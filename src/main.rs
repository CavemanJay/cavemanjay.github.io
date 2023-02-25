#![warn(clippy::pedantic, clippy::nursery)]
mod components;

use components::{Home, NotFound};
use std::process::Command;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq, Copy)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Projects => html! {},
        Route::NotFound => html! {<NotFound/>},
    }
}

#[function_component]
fn App() -> Html {
    let onchange = move |e: Event| {
        let input = e.target_dyn_into::<HtmlInputElement>().unwrap();
        let val = input.value();
        let x = Command::new(val).spawn().unwrap().wait().unwrap();
    };

    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
            <footer class="flex pb-2">
                <div class="flex-1 text-left pl-3">
                    <label class="block">{"Potential vulnerability"}</label>
                    <input {onchange} />
                </div>
                <div class="flex-1 text-center">
                    <span>{"Created using: "}</span>
                        <img class="w-8 inline" src="/static/rustacean-flat-gesture.svg" type="image/svg+xml" />
                    <a target="_blank" href="https://www.rust-lang.org/">
                        {"rust"}
                    </a>
                    <span>{" + "}</span>
                    <a class="underline" target="_blank" href="https://tailwindcss.com/">
                        {"tailwindcss"}
                    </a>
                </div>
                <div class="flex-1 text-right pr-3">
                    <a class="underline" target="_blank" href="https://github.com/cavemanjay/cavemanjay.github.io">
                        {"Source"}
                    </a>
                </div>
            </footer>
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
