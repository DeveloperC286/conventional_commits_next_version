use std::collections::VecDeque;

use rstest::rstest;
use semver::Version;

use crate::calculation_mode::CalculationMode;
use crate::commits::commit::Commit;
use crate::commits::Commits;

mod commit;
mod commits;
