use std::collections::VecDeque;

use rstest::rstest;
use semver::Version;

use crate::calculation_mode::CalculationMode;
use crate::commits::Commits;
use crate::commits::commit::Commit;

mod commit;
mod commits;
