use jsfuckrs;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, EventTarget, HtmlInputElement, HtmlTextAreaElement, Node};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct JsfVisualizerProps {}

// TODO: Give ability for user to both console.log and alert output

#[function_component]
pub fn JsfVisualizer(props: &JsfVisualizerProps) -> Html {
    let input_value_handle = use_state(|| AttrValue::from("alert(1)"));
    let input_value = (*input_value_handle).clone();
    let input_value_compile = (*input_value_handle).clone();
    let output_value_handle = use_state(String::new);
    let output_value_exe = (*output_value_handle).clone();
    let output_value = (*output_value_handle).clone();

    let execute = move |_| {
        let y = js_sys::eval(&output_value_exe).unwrap();
        gloo_console::log!(y);
    };
    let compile = move |_| {
        let src = input_value_compile.clone();
        if src.trim() == "" {
            return;
        }

        let compiled = jsfuckrs::lbp::compile(src);
        output_value_handle.set(compiled);
    };
    let onchange = Callback::from(move |e: Event| {
        let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value().into());
    });
    html! {
        <div>
            <input class="m-10 [writing-mode:horizontal-tb]" {onchange} value={input_value.clone()} />
            <button class="border rounded p-2" onclick={compile}>{"Compile"}</button>
            <hr/>
            <div class="max-w-lg h-80 overflow-auto border border-[var(--secondary)] rounded m-2">
                <code>{&output_value}</code>
            </div>
            <button class="border mx-2 rounded p-2 border-[var(--secondary)]" onclick={execute}>{"Run This"}</button>
        </div>
    }
}
