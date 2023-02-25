use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
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
                        {"blogs"}
                    </li>
                    <li>
                        {"projects"}
                    </li>
                </ul>
            </div>
        </>
    }
}
