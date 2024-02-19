//! `fav_utils` is a utility library for [Fav](https://github.com/kingwingfly/fav).

#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]

mod proto;

#[cfg(feature = "bili")]
pub mod bili;
#[cfg(feature = "bili")]
pub use bili::*;

pub mod error;
mod utils;
pub use error::*;
