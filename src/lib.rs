#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

mod components;
mod pages;

import!(pub, components, App);
use pages::*;

/// Macro to declare a component and its props.
///
/// Similar to the concept of [barrels](https://basarat.gitbook.io/typescript/main-1/barrel) in typescript and javascript
///
/// Makes use of the [paste](https://docs.rs/paste/) crate to also export the props type.
/// Reference: <https://users.rust-lang.org/t/concatenating-type-name-with-suffix-in-macros/47738/3>
#[macro_export]
macro_rules! declare {
    ($mod_name:ident, $component:ident) => {
        mod $mod_name;
        // pub use $mod_name::{$component, [<$component Props>]};
        $crate::import!(pub, $mod_name, $component);
    };
}

#[macro_export]
macro_rules! import {
    ($component:ident) => {
        paste::paste! {
            use $crate::{$component, [<$component Props>]};
        }
    };
    ([$($component:ident),*]) => {
        paste::paste! {
            $(
            use $crate::{$component, [<$component Props>]};
            )*
        }
    };
    ($mod_name:ident, $component:ident) => {
        paste::paste! {
            use $mod_name::{$component, [<$component Props>]};
        }
    };
    (pub, $mod_name:ident, $component:ident) => {
        paste::paste! {
            pub use $mod_name::{$component, [<$component Props>]};
        }
    };
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Page {
    #[default]
    Root,
    Home,
    Projects,
    About,
    JSF___,
    BrainF___,
    TrueJson,
}

impl Page {
    #[must_use]
    pub const fn path(self) -> &'static str {
        match self {
            Self::Root => "/",
            Self::Home => "/home",
            Self::About => "/about",
            Self::Projects => "/projects",
            Self::JSF___ => "/projects/esolangs/jsf***",
            Self::BrainF___ => "/projects/esolangs/brainf***",
            Self::TrueJson => "/projects/misc/truejson",
        }
    }
}
