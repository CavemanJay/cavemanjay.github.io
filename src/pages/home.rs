use crate::Page;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // let _navigator = use_navigator().unwrap();
    view! { cx,
        <>
            <div class="w">
                <header>
                    <h1 class="text-[2em] font-[monospace] font-bold">"/optsec"</h1>
                    <p></p>
                </header>
                <ul>
                    <li>
                        "# whoami"
                    </li>
                    <ul>
                        <li>"Jay C*****"</li>
                        <ul>
                            <li>"Software Developer / Casual Hacker"</li>
                        </ul>
                    </ul>
                    <li>
                        <A class="link" href="/about">"about me"</A>
                    </li>
                    <li>
                        "projects"
                        <ul>
                            <li>
                                "Esoteric Programming Languages"
                                <ul>
                                    <li>
                                        // <A href="/projects/esolangs/brainf***" class="link font-bold">{"BrainF*** Visualizer"}</A>
                                        <A href=Page::BrainF___.path() class="link font-bold">"BrainF*** Visualizer"</A>
                                    </li>
                                    <li>
                                        // <A href="/projects/esolangs/jsf***" class="link font-bold">{"JSF*** Transpiler"}</A>
                                        <A href=Page::JSF___.path() class="link font-bold">"JSF*** Transpiler"</A>
                                    </li>
                                </ul>
                            </li>
                            <li>
                                "Misc"
                                <ul>
                                    <li>
                                        <A href=Page::TrueJson.path() class="link font-bold">"Json to TrueJson Converter"</A>
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
