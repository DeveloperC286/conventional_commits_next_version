use strum_macros::{Display, EnumString};

/// The mode of calculation to use on the range of Commits to calculate the next semantic version.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, EnumString)]
pub enum CalculationMode {
    /// In batch mode the largest Semantic Versioning increment determined by the Conventional
    /// Commits type across all the commits is the only increment applied.
    Batch,
    /// In consecutive mode each Git commit in the Conventional Commits specification is applied to Semantic Versioning calculation in chronological or      der.
    Consecutive,
}
