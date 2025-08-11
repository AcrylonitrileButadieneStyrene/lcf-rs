#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_possible_truncation)]

mod formats;
pub mod helpers;
mod macros;

pub use formats::*;
