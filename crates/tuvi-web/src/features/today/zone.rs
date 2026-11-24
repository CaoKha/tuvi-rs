//! The visitor's clock for the day plan. `zone.js` keeps their offset from
//! UTC in the `tz` cookie, in minutes east of UTC (Vietnam is 420), and the
//! canh giờ are labelled in it. The chart itself stays Vietnam time: the
//! engine reads the birth moment and the day as Vietnam time.

use topcoat::{
    context::Cx,
    cookie::{Cookies, cookies},
};

/// The cookie that keeps the visitor's offset from UTC, in minutes.
pub(super) const COOKIE: &str = "tz";

/// Vietnam's offset from UTC, in minutes.
const VIETNAM: i32 = 7 * 60;

/// The widest real offsets, UTC−12:00 to UTC+14:00.
const MIN_OFFSET: i32 = -12 * 60;
const MAX_OFFSET: i32 = 14 * 60;

/// A visitor's clock offset from UTC, in minutes east.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Zone {
    offset: i32,
}

impl Zone {
    /// The zone of a request: the `tz` cookie, else Vietnam.
    pub(super) fn of(cx: &Cx) -> Self {
        let value = cookies(cx).get(COOKIE).map(|cookie| cookie.value().to_owned());
        Self::parse(value.as_deref()).unwrap_or_default()
    }

    /// The zone of a cookie value, when it is a real offset.
    fn parse(value: Option<&str>) -> Option<Self> {
        value?
            .parse::<i32>()
            .ok()
            .filter(|offset| (MIN_OFFSET..=MAX_OFFSET).contains(offset))
            .map(|offset| Self { offset })
    }

    /// The clock label of a canh giờ that starts at `hour` in Vietnam, such
    /// as `07:00–09:00`. Half and quarter hour zones keep their minutes.
    pub(super) fn label(self, hour: u8) -> String {
        let (from_hour, from_minute) = self.clock(hour);
        let (to_hour, to_minute) = self.clock(hour + 2);
        format!("{from_hour:02}:{from_minute:02}–{to_hour:02}:{to_minute:02}")
    }

    /// The hour and minute of a Vietnam hour in this zone, 0–23 and 0–59.
    fn clock(self, hour: u8) -> (i32, i32) {
        let minutes = (i32::from(hour) * 60 - VIETNAM + self.offset).rem_euclid(24 * 60);
        (minutes / 60, minutes % 60)
    }
}

impl Default for Zone {
    fn default() -> Self {
        Self { offset: VIETNAM }
    }
}

#[cfg(test)]
mod tests {
    use super::Zone;

    #[test]
    fn vietnam_keeps_the_canh_gio_hours() {
        let zone = Zone::default();
        assert_eq!(zone.label(23), "23:00–01:00");
        assert_eq!(zone.label(7), "07:00–09:00");
    }

    #[test]
    fn labels_follow_the_visitor_offset() {
        assert_eq!(Zone::parse(Some("-300")).unwrap().label(23), "11:00–13:00");
        assert_eq!(Zone::parse(Some("540")).unwrap().label(23), "01:00–03:00");
    }

    #[test]
    fn quarter_hour_zones_keep_their_minutes() {
        // Kathmandu, UTC+05:45.
        assert_eq!(Zone::parse(Some("345")).unwrap().label(7), "05:45–07:45");
    }

    #[test]
    fn only_real_offsets_parse() {
        assert_eq!(Zone::parse(None), None);
        assert_eq!(Zone::parse(Some("")), None);
        assert_eq!(Zone::parse(Some("north")), None);
        assert_eq!(Zone::parse(Some("-721")), None);
        assert_eq!(Zone::parse(Some("841")), None);
        assert!(Zone::parse(Some("-720")).is_some());
        assert!(Zone::parse(Some("840")).is_some());
    }
}
