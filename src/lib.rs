#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_possible_truncation)]
#![feature(iterator_try_collect)]
#![feature(iter_array_chunks)]

//! # Lcf-rs
//!
//! Lcf is the file format used by the RPG Maker 2000/2003 game engine.
//!
//! Each of the different file types is in a module with its extension as the name:
//! - [`crate::ldb::LcfDataBase`] e.g. `RPG_RT.ldb`
//! - [`crate::lmt::LcfMapTree`] e.g. `RPG_RT.lmt`
//! - [`crate::lmu::LcfMapUnit`] e.g. `MapXXXX.lmu`
//! - [`crate::lsd::LcfSaveData`] e.g. `SaveXX.lsd`
//!
//! The [`crate::Lcf`] enum contains all of the above 4 types as variants and can be used if the exact one is not known ahead of time.
//!
//! ---
//!
//! The above 5 types are wrappers around their corresponding structs in the [`crate::raw`] module:
//! - [`crate::raw::RawLcf`]
//! - [`crate::raw::ldb::RawLcfDataBase`]
//! - [`crate::raw::lmt::RawLcfMapTree`]
//! - [`crate::raw::lmu::RawLcfMapUnit`]
//! - [`crate::raw::lsd::RawLcfSaveData`]
//!
//! All of the raw structs implement [`binrw::BinRead`] and [`binrw::BinWrite`].
//!
//! ## Example
//! ```no_run
//! let bytes = std::fs::read("RPG_RT.ldb").expect("file exists");
//! let mut reader = std::io::Cursor::new(bytes);
//! let database = lcf::Lcf::read(&mut reader).expect("valid lcf file");
//! assert!(matches!(database, lcf::Lcf::DataBase(_)))
//! ```

mod macros;

pub mod enums;
pub mod helpers;

mod formats;
pub use formats::*;
