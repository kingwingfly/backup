//! Attribute,
//! managing the resources' attributes

use std::str::FromStr;

#[cfg(feature = "derive")]
pub use fav_derive::Attr;

/// The resource's id.
/// # Example
/// ```
/// # use fav_core::attr::Id;
/// let id: Id = 123.into();
/// assert_eq!(id, Id::I32(123));
/// let id: Id = "123".parse().unwrap();
/// assert_eq!(id, Id::I32(123));
/// let id: Id = "68719476735".parse().unwrap();
/// assert_eq!(id, Id::I64(68719476735)); // 68719476735 is 0xFFFF_FFFF_F, which > i32::MAX
/// let id: Id = "abc".parse().unwrap();
/// assert_eq!(id, Id::String("abc".to_owned()));
/// ```
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum Id {
    I64(i64),
    I32(i32),
    String(String),
}

impl From<i64> for Id {
    fn from(id: i64) -> Self {
        Id::I64(id)
    }
}

impl From<i32> for Id {
    fn from(id: i32) -> Self {
        Id::I32(id)
    }
}

impl FromStr for Id {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i32>() {
            Ok(id) => Id::I32(id),
            Err(_) => match s.parse::<i64>() {
                Ok(id) => Id::I64(id),
                Err(_) => Id::String(s.to_owned()),
            },
        })
    }
}

/// Basical attributes
/// #Example
/// ```
/// # #[path = "test_utils/mod.rs"]
/// # mod test_utils;
/// # use test_utils::data::AttrTest;
/// # use fav_core::attr::{Attr, Id};
///
/// impl Attr for AttrTest {
///     fn id(&self) -> Id {
///        self.id.into()
///     }
///
///     fn name(&self) -> &str {
///         &self.name
///     }
/// }
///
/// # fn main() {
/// let res = AttrTest::default();
///
/// assert_eq!(res.id(), 0.into());
/// assert_eq!(res.name(), "");
/// # }
/// ```
/// Derive macros example:
/// ```
/// # use fav_core::attr::{Attr, Id};
/// #[derive(Attr)]
/// struct AttrTest {
///     id: i32,
///     name: String,
/// }
/// ```
pub trait Attr {
    /// Return the id of the target
    fn id(&self) -> Id;
    /// Return the name of the target
    fn name(&self) -> &str;
}
