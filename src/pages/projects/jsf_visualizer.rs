use jsfuckrs;
use leptos::*;

#[component]
pub fn JsfVisualizer(cx: Scope) -> impl IntoView {
    let (src, set_src) = create_signal(cx, String::from("alert(1)"));
    let (compiled, set_compiled) = create_signal(cx, String::new());
    // let js_state = use_state(|| AttrValue::from("alert(1)"));
    // let output_value = use_state(String::new);

    let compile = move |_| {
        let src = src();
        if src.trim() == "" {
            return;
        }
        let result = jsfuckrs::lbp::compile(src);
        set_compiled.set(result);
    };
    let execute = move |_| {
        let y = js_sys::eval(&compiled()).unwrap();
        log::info!("{:?}", y);
    };

    view! {
        cx,
        <div>
            <input
                class="m-10 [writing-mode:horizontal-tb]"
                value=src
                on:change = move |ev| {
                    let val = event_target_value(&ev);
                    set_src.update(|v|*v = val);
                }
            />
            <button
                class="border rounded p-2"
                on:click=compile
            >
                {"Compile"}
            </button>
            <hr/>
            <div class="max-w-lg h-80 overflow-auto border border-[var(--secondary)] rounded m-2">
                <code>{compiled}</code>
            </div>
            <button
                class="border mx-2 rounded p-2 border-[var(--secondary)]"
                on:click=execute
            >
                {"Run This"}
            </button>
        </div>
    }
}
