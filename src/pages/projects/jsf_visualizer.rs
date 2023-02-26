use jsfuckrs;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, EventTarget, HtmlInputElement, HtmlTextAreaElement, Node};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct JsfVisualizerProps {}

// TODO: Give ability for user to both console.log and alert output

#[function_component]
pub fn JsfVisualizer(props: &JsfVisualizerProps) -> Html {
    let js_state = use_state(|| AttrValue::from("alert(1)"));
    let output_value = use_state(String::new);

    let execute = {
        let output_value = output_value.clone();
        Callback::from(move |_| {
            let y = js_sys::eval(&output_value).unwrap();
            gloo_console::log!(y);
        })
    };
    let compile = {
        let src = js_state.clone();
        let output = output_value.clone();
        Callback::from(move |_| {
            if src.trim() == "" {
                return;
            }

            let compiled = jsfuckrs::lbp::compile(&*src);
            output.set(compiled);
        })
    };
    let onchange = {
        let js_state = js_state.clone();
        Callback::from(move |e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            js_state.set(target.unchecked_into::<HtmlInputElement>().value().into());
        })
    };

    html! {
        <div>
            <input class="m-10 [writing-mode:horizontal-tb]" {onchange} value={js_state.as_str().to_string()} />
            <button class="border rounded p-2" onclick={compile}>{"Compile"}</button>
            <hr/>
            <div class="max-w-lg h-80 overflow-auto border border-[var(--secondary)] rounded m-2">
                <code>{&*output_value}</code>
            </div>
            <button class="border mx-2 rounded p-2 border-[var(--secondary)]" onclick={execute}>{"Run This"}</button>
        </div>
    }
}
