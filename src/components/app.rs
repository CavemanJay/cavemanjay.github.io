use crate::{pages::*, Page};
use leptos::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {

    view! { cx,
        <div class="h-[96vh] overflow-auto container mx-auto">
            <Router>
                // <nav>
                //     <A exact=true href=Page::Home.path()>"Home"</A>
                //     <A exact=true href=Page::About.path()>"About"</A>
                //     <A href=Page::Projects.path()>"Projects"</A>
                // </nav>
                <main class="">
                    <Routes>
                        <Route
                            path=Page::Root.path()
                            view=move |cx| view! { cx,  <Redirect path=Page::Home.path() /> }
                        />
                        <Route
                            // path="/home"
                            path=Page::Home.path()
                            view=move |cx| view! { cx,  <Home/> }
                        />
                        <Route
                            // path="/about"
                            path=Page::About.path()
                            view=move |cx| view! { cx,  <About/> }
                        />
                        <Route
                            path=Page::BrainF___.path()
                            view=move |cx| view! { cx,  <UnderConstruction /> }
                        />
                        <Route
                            path=Page::JSF___.path()
                            view=move |cx| view! { cx,  <JsfVisualizer /> }
                        />
                        <Route
                            path=Page::TrueJson.path()
                            view=move |cx| view! { cx,  <UnderConstruction /> }
                        />
                        <Route
                            path="*"
                            view=move |cx| view! { cx,  <NotFound/> }
                        />
                    </Routes>
                </main>
                <footer class="flex h-[3vh] bg-[var(--background)]">
                    <div class="flex-1 text-left pl-3">
                    </div>
                    <div class="flex-grow-[2] text-center">
                        <span>"Created using: "</span>
                        <a class="link" target="_blank" href="https://www.rust-lang.org/">
                            <img class="w-8 inline" src="/static/rustacean-flat-gesture.svg" type="image/svg+xml" />
                        </a>
                        <span>" + "</span>
                        <a class="link" target="_blank" href="https://github.com/leptos-rs/leptos">
                            "leptos"
                        </a>
                        <span>" + "</span>
                        <a class="link" target="_blank" href="https://tailwindcss.com/">
                            "tailwindcss"
                        </a>
                    </div>
                    <div class="flex-1 text-right pr-3">
                        <a class="link" target="_blank" href="https://github.com/cavemanjay/cavemanjay.github.io">
                            "Source"
                        </a>
                    </div>
                </footer>
            </Router>
        </div>
    }
}
