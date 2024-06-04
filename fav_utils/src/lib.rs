#![doc = include_str!("../README.md")]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "bili")]
pub mod bili;
#[cfg(feature = "bili")]
pub use bili::*;

pub mod error;
mod utils;
pub use error::*;
