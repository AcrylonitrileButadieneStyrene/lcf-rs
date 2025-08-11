#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_possible_truncation)]

//! # Lcf-rs
//!
//! Lcf is the file format used by the RPG Maker 2000/2003 game engine.
//!
//! Each of the different file types is in a module with its extension as the name:
//! - [`ldb::LcfDataBase`] e.g. `RPG_RT.ldb`
//! - [`lmt::LcfMapTree`] e.g. `RPG_RT.lmt`
//! - [`lmu::LcfMapUnit`] e.g. `MapXXXX.lmu`
//! - [`lsd::LcfSaveData`] e.g. `SaveXX.lsd`
//!
//! The [`Lcf`] enum contains all of the above 4 types as variants and can be used if the exact one is not known ahead of time.
//!
//! Each of the above 5 types contains a [`read`](Lcf::read) and [`write`](Lcf::write) method, as well as the [`binrw::BinRead`] and [`binrw::BinWrite`] traits.
//!
//! ## Example
//! ```no_run
//! let bytes = std::fs::read("RPG_RT.ldb").expect("file exists");
//! let mut reader = std::io::Cursor::new(bytes);
//! let database = lcf::Lcf::read(&mut reader).expect("valid lcf file");
//! assert!(matches!(database, lcf::Lcf::DataBase(_)))
//! ```

mod formats;
pub mod helpers;
mod macros;

pub use formats::*;
