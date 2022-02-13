#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod calculation_mode;
mod commits;
mod git_history_mode;

pub use crate::calculation_mode::CalculationMode;
pub use crate::commits::Commits;
pub use crate::git_history_mode::GitHistoryMode;
