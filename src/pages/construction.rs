use leptos::*;


#[component]
pub fn UnderConstruction(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="w-full m-0 absolute top-[50%] translate-y-[-50%]">
            <p class="text-center text-2xl">{"Uh oh..."}</p>
            <p class="text-center secondary">{"I haven't quite finished this page yet. Check back later :)"}</p>
            <img class="w-[50%]" src="/static/yikes.jpg"/>
        </div>
    }
}
