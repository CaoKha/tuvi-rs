//! The birth form: what the browser sends, how it is checked, and the card
//! name.
//!
//! The date comes from three wheels and the hour from a 12-hour clock with
//! an AM/PM toggle, the way people know their birth time.

use serde::Deserialize;
use topcoat::context::Cx;
use tuvi_core::{BirthInput, Chart, ChartError, Gender, SolarYear};

use crate::{i18n::Lang, t};

/// The half of the day on the 12-hour clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Buoi {
    #[serde(rename = "AM")]
    Sang,
    #[serde(rename = "PM")]
    Chieu,
}

impl Buoi {
    /// The form value.
    pub fn value(self) -> &'static str {
        match self {
            Self::Sang => "AM",
            Self::Chieu => "PM",
        }
    }
}

/// The form value of a gender.
pub fn gender_value(gender: Gender) -> &'static str {
    match gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nu",
    }
}

/// The hour 0–23 for a clock hour 1–12. 12 AM is 0 and 12 PM is 12.
/// Returns `None` when `gio` is not 1–12.
pub fn hour_24(gio: u8, buoi: Buoi) -> Option<u8> {
    if !(1..=12).contains(&gio) {
        return None;
    }
    let base = gio % 12;
    Some(match buoi {
        Buoi::Sang => base,
        Buoi::Chieu => base + 12,
    })
}

/// The form fields, as the browser sends them.
#[derive(Debug, Clone, Deserialize)]
pub struct BirthForm {
    pub day: u8,
    pub month: u8,
    pub year: SolarYear,
    /// Clock hour 1–12. Missing until the user picks one.
    #[serde(default)]
    pub gio: Option<u8>,
    pub buoi: Buoi,
    /// Missing until the user picks one.
    #[serde(default)]
    pub gender: Option<Gender>,
}

/// A user mistake.
#[derive(Debug, PartialEq, Eq)]
pub enum FormError {
    NoHour,
    NoGender,
    Chart(ChartError),
}

impl FormError {
    /// The message for the page, in the request's language.
    pub fn message(&self, cx: &Cx) -> String {
        match self {
            Self::NoHour => t!(cast_error_no_hour).resolve(cx),
            Self::NoGender => t!(cast_error_no_gender).resolve(cx),
            Self::Chart(ChartError::InvalidDate) => t!(cast_error_invalid_date).resolve(cx),
            Self::Chart(ChartError::YearOutOfRange) => t!(cast_error_year_range).resolve(cx),
        }
        .into_owned()
    }
}

impl BirthForm {
    /// Checks the fields and builds the chart once, so a bad birth moment
    /// never reaches a URL.
    pub fn validate(&self) -> Result<BirthInput, FormError> {
        let hour = self
            .gio
            .and_then(|gio| hour_24(gio, self.buoi))
            .ok_or(FormError::NoHour)?;
        let gender = self.gender.ok_or(FormError::NoGender)?;
        let birth = BirthInput {
            year: self.year,
            month: self.month,
            day: self.day,
            hour,
            gender,
        };
        tuvi_core::build_chart(birth).map_err(FormError::Chart)?;
        Ok(birth)
    }
}

/// The card's name, as Tử Vi books name a chart: gender and the Can Chi of
/// the lunar year, for example "Nữ mạng Bính Tý" or "Female · Bính Tý year".
pub fn card_name(chart: &Chart, lang: Lang) -> String {
    let (can, chi) = (chart.nam_can, chart.nam_chi);
    match (lang, chart.input.gender) {
        (Lang::En, Gender::Nam) => format!("Male · {can} {chi} year"),
        (Lang::En, Gender::Nu) => format!("Female · {can} {chi} year"),
        (Lang::Vi, Gender::Nam) => format!("Nam mạng {can} {chi}"),
        (Lang::Vi, Gender::Nu) => format!("Nữ mạng {can} {chi}"),
    }
}

#[cfg(test)]
mod tests {
    use tuvi_core::build_chart;

    use super::*;

    fn form(body: &str) -> BirthForm {
        serde_urlencoded::from_str(body).unwrap()
    }

    const FULL: &str = "day=15&month=4&year=1996&gio=10&buoi=PM&gender=Nu";

    #[test]
    fn full_form_is_valid() {
        assert_eq!(
            form(FULL).validate(),
            Ok(BirthInput {
                year: SolarYear(1996),
                month: 4,
                day: 15,
                hour: 22,
                gender: Gender::Nu,
            })
        );
    }

    #[test]
    fn clock_hour_maps_to_24_hours() {
        assert_eq!(hour_24(12, Buoi::Sang), Some(0));
        assert_eq!(hour_24(1, Buoi::Sang), Some(1));
        assert_eq!(hour_24(11, Buoi::Sang), Some(11));
        assert_eq!(hour_24(12, Buoi::Chieu), Some(12));
        assert_eq!(hour_24(11, Buoi::Chieu), Some(23));
        assert_eq!(hour_24(0, Buoi::Sang), None);
        assert_eq!(hour_24(13, Buoi::Chieu), None);
    }

    #[test]
    fn missing_hour_or_gender_is_an_error() {
        let no_hour = form("day=1&month=1&year=2000&buoi=PM&gender=Nam");
        assert_eq!(no_hour.validate(), Err(FormError::NoHour));
        let bad_hour = form("day=1&month=1&year=2000&gio=13&buoi=PM&gender=Nam");
        assert_eq!(bad_hour.validate(), Err(FormError::NoHour));
        let no_gender = form("day=1&month=1&year=2000&gio=3&buoi=PM");
        assert_eq!(no_gender.validate(), Err(FormError::NoGender));
    }

    #[test]
    fn impossible_dates_are_errors() {
        let feb_30 = form("day=30&month=2&year=2000&gio=3&buoi=PM&gender=Nam");
        assert_eq!(
            feb_30.validate(),
            Err(FormError::Chart(ChartError::InvalidDate))
        );
        let too_late = form("day=1&month=1&year=2300&gio=3&buoi=PM&gender=Nam");
        assert_eq!(
            too_late.validate(),
            Err(FormError::Chart(ChartError::YearOutOfRange))
        );
    }

    #[test]
    fn name_uses_gender_and_year_can_chi() {
        let chart = build_chart(form(FULL).validate().unwrap()).unwrap();
        assert_eq!(card_name(&chart, Lang::Vi), "Nữ mạng Bính Tý");
        assert_eq!(card_name(&chart, Lang::En), "Female · Bính Tý year");
    }
}
