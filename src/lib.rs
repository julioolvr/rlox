#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate cfg_if;

mod rlox;
pub use rlox::api::*;
pub use rlox::wasm::*;
