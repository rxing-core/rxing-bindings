#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod barcode_format;
mod encode;
mod decode;

pub use barcode_format::*;
pub use encode::*;
pub use decode::*;
