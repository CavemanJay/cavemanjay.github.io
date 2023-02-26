#![warn(clippy::pedantic, clippy::nursery)]
mod components;
mod pages;

use crate::{components::*, pages::*};
use yew::prelude::*;
use yew_router::prelude::*;

#[macro_export]
macro_rules! declare {
    ($mod:ident,$component:ident) => {
        mod $mod;
        pub use $mod::$component;
    };
}

type RouteLink = Link<Route>;

#[derive(Clone, Routable, PartialEq, Copy)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/projects/brainf***")]
    Brainfuck,
    #[at("/projects/jsf***")]
    Jsfuck,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    // let x = LinearEngine::run(">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.").unwrap();
    match routes {
        Route::Home => html! {<Home/>},
        // Route::Brainfuck => html! {<BfVisualizer/>},
        Route::Jsfuck => html! {<JsfVisualizer/>},
        Route::Projects | Route::Brainfuck => html! {<UnderConstruction/>},
        Route::NotFound => html! {<NotFound/>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
