// use wasm_bindgen::UnwrapThrowExt;
// use web_sys::window;
// use yew::prelude::*;

use leptos::*;


#[component]
pub fn NotFound(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <h1 class="text-9xl text-center">{"404"}</h1>
            <div class="text-center [&>p]:leading-3">
                <p class="text-lg text-[#87f087]">{"I have no idea what you are looking for..."}</p>
                <p class="text-lg text-[#87f087]">{"Or why you are looking for it..."}</p>
                <p class="text-lg text-[#87f087]">{"But I promise it's not here."}</p>
                <p class="text-lg text-[#87f087]">{"Feel free to keep looking though :)"}</p>
                // <button {onclick} class="mt-8 p-2 rounded-lg border-green-500 hover:bg-green-600 border">{"Take me back!"}</button>
            </div>
        </div>
    }
}
