use crate::declare;
pub mod projects;
pub use projects::*;

declare!(home, Home);
declare!(not_found, NotFound);
declare!(construction, UnderConstruction);
declare!(about, About);