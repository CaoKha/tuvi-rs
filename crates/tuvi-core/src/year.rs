//! Years of the two calendars. A lunar year starts at Tết, so it overlaps
//! two solar years: a date in January can be in the previous lunar year.
//! Separate types keep one from being passed for the other.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A year of the Gregorian (solar) calendar, e.g. `SolarYear(1996)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SolarYear(pub i32);

/// A year of the Vietnamese lunar calendar, named by the solar year in which
/// its Tết falls, e.g. `LunarYear(1996)` is Bính Tý, from 19/02/1996 to
/// 06/02/1997.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LunarYear(pub i32);

impl fmt::Display for SolarYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for LunarYear {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
