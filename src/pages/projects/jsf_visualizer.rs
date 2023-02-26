use jsfuckrs;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct JsfVisualizerProps {}

#[function_component]
pub fn JsfVisualizer(props: &JsfVisualizerProps) -> Html {
    let input_value_handle = use_state(|| AttrValue::from("alert(1)"));
    let input_value = (*input_value_handle).clone();
    let input_value_compile = (*input_value_handle).clone();
    let output_value_handle = use_state(String::new);
    let output_value = (*output_value_handle).clone();

    let compile = move |_| {
        let compiled = jsfuckrs::lbp::compile(input_value_compile.clone());
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
            // <label>{&input_value}</label>
            <input class="m-10 [writing-mode:horizontal-tb]" {onchange} value={input_value.clone()} />
            <button class="border rounded p-2" onclick={compile}>{"Compile"}</button>
            <hr/>
            <div class="max-w-lg max-h-80 overflow-auto">
                <code>{&output_value}</code>
            </div>
        </div>
    }
}
