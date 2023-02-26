use yew::prelude::*;
use yew_router::prelude::*;

use crate::{Route, RouteLink};

#[function_component]
pub fn Home() -> Html {
    let navigator = use_navigator().unwrap();
    // let bf_navigate = move |_| navigator.push(&Route::Brainfuck);
    // let jsf_navigate = move |_| navigator.push(&Route::JsFuck);
    html! {
        <>
            <div class="w">
                <header>
                    <h1 class="text-[2em] font-[monospace] font-bold">{"/optsec"}</h1>
                    <p></p>
                </header>
                <ul>
                    <li>
                        {"# whoami"}
                    </li>
                    <ul>
                        <li>{"Jay C*****"}</li>
                        <ul>
                            <li>{"Software Developer / Casual Hacker"}</li>
                        </ul>
                    </ul>
                    <li>
                        {"languages"}
                        <ol class="[&>li>span]:secondary">
                            <li><span>{"Rust"}</span></li>
                            <li><span>{"Python"}</span></li>
                            <li><span>{"C#"}</span></li>
                            <li><span>{"Go"}</span></li>
                            <li><span>{"F#"}</span></li>
                            <li><span>{"Haskell"}</span></li>
                        </ol>
                    </li>
                    <li>
                        {"projects"}
                        <ul>
                            <li>
                                {"Esoteric Programming Language Visualizers"}
                                <ul>
                                    <li>
                                        <RouteLink to={Route::Brainfuck} classes="link font-bold">{"Brainf***"}</RouteLink>
                                    </li>
                                    <li>
                                        <RouteLink to={Route::Jsfuck} classes="link font-bold">{"JsF***"}</RouteLink>
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </li>
                </ul>
            </div>
        </>
    }
}
