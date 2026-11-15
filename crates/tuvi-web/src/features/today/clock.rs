//! Today in Vietnam (UTC+7, no daylight saving), from the system clock.

use std::time::{SystemTime, UNIX_EPOCH};

use tuvi_core::SolarYear;

/// Vietnam's offset from UTC, in seconds.
const VIETNAM_OFFSET: i64 = 7 * 3600;

/// A day and the hour 0–23 in Vietnam.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Now {
    pub year: SolarYear,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
}

impl Now {
    /// The current day and hour in Vietnam.
    pub(super) fn vietnam() -> Self {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| i64::try_from(since.as_secs()).unwrap_or(0));
        Self::at(seconds)
    }

    /// The day and hour in Vietnam at `unix` seconds.
    fn at(unix: i64) -> Self {
        let local = unix + VIETNAM_OFFSET;
        let (year, month, day) = civil_from_days(local.div_euclid(86_400));
        let hour = local.rem_euclid(86_400) / 3600;
        Self {
            year: SolarYear(year),
            month,
            day,
            hour: hour as u8,
        }
    }

    /// The canh giờ of the hour: 0 for Tý (23–1h), 1 for Sửu, …
    pub(super) fn canh(self) -> usize {
        usize::from(self.hour).div_ceil(2) % 12
    }

    /// The day the current canh giờ belongs to. Giờ Tý starts at 23h and
    /// belongs to the next day, as in the engine.
    pub(super) fn tu_vi_day(self) -> (SolarYear, u8, u8) {
        if self.hour == 23 {
            let days = days_from_civil(self.year.0, self.month, self.day) + 1;
            let (year, month, day) = civil_from_days(days);
            (SolarYear(year), month, day)
        } else {
            (self.year, self.month, self.day)
        }
    }
}

/// The Gregorian date of a day count from 1970-01-01 (Howard Hinnant's
/// `civil_from_days`).
fn civil_from_days(days: i64) -> (i32, u8, u8) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
    let year = (yoe + era * 400 + i64::from(month <= 2)) as i32;
    (year, month, day)
}

/// The day count from 1970-01-01 of a Gregorian date.
fn days_from_civil(year: i32, month: u8, day: u8) -> i64 {
    let year = i64::from(year) - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let yoe = year.rem_euclid(400);
    let month = i64::from(month);
    let doy = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vietnam_time_from_unix_seconds() {
        // 2026-11-15T10:30:00Z is 17:30 in Vietnam.
        let now = Now::at(1_794_738_600);
        assert_eq!(
            (now.year, now.month, now.day, now.hour),
            (SolarYear(2026), 11, 15, 17)
        );
        assert_eq!(now.canh(), 9); // Dậu, 17–19h.
        // 2026-12-31T17:00:00Z is 00:00 on 1 January in Vietnam.
        let new_year = Now::at(1_798_736_400);
        assert_eq!(
            (new_year.year, new_year.month, new_year.day),
            (SolarYear(2027), 1, 1)
        );
    }

    #[test]
    fn gio_ty_at_23h_belongs_to_the_next_day() {
        let late = Now {
            year: SolarYear(2026),
            month: 12,
            day: 31,
            hour: 23,
        };
        assert_eq!(late.canh(), 0);
        assert_eq!(late.tu_vi_day(), (SolarYear(2027), 1, 1));
    }

    #[test]
    fn day_counts_round_trip() {
        for days in [-1, 0, 59, 10_957, 20_772, 47_482] {
            let (year, month, day) = civil_from_days(days);
            assert_eq!(days_from_civil(year, month, day), days);
        }
        assert_eq!(civil_from_days(0), (1970, 1, 1));
    }
}
