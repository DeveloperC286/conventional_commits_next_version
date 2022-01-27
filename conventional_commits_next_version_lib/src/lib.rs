#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod commits;

pub use crate::commits::Commits;
