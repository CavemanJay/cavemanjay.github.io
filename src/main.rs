#![warn(clippy::pedantic, clippy::nursery)]
use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

// mod projects;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 1);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            // log!("test");
            let value = *counter * 2;
            counter.set(value);
        }
    };

    html! {
        <div class="w">
            <header>
                <h1>{"/optsec"}</h1>
                <p></p>
            </header>
            <ul>
                <li>
                    {"# whoami"}
                </li>
                <ul>
                    <li>{"Jay C*****"}</li>
                    <ul>
                        <li>{"Software Developer"}</li>
                    </ul>
                </ul>
            </ul>
            <button class={classes!("border-2", "rounded")} {onclick}>{"test button"}</button>
            <p>{*counter}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
