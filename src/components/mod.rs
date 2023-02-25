macro_rules! declare {
    ($mod:ident,$component:ident) => {
        mod $mod;
        pub use $mod::$component;
    };
}

declare!(home, Home);
declare!(not_found, NotFound);
