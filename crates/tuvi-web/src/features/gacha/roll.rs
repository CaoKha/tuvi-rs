//! The draw: the user picks the date, the draw picks the hour and gender.

use rand::Rng;
use serde::Deserialize;
use tuvi_core::{BirthInput, Chart, ChartError, Gender, SolarYear, build_chart};

use topcoat::context::Cx;

use super::CardCode;
use crate::{i18n::Lang, t};

/// A solar birth date, as the draw form sends it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct BirthDate {
    pub day: u8,
    pub month: u8,
    pub year: SolarYear,
}

impl BirthDate {
    /// The date of a card.
    pub fn of(birth: BirthInput) -> Self {
        Self {
            day: birth.day,
            month: birth.month,
            year: birth.year,
        }
    }
}

/// A date the engine cannot use.
#[derive(Debug, PartialEq, Eq)]
pub struct BadDate(ChartError);

impl BadDate {
    /// The message for the page, in the request's language.
    pub fn message(&self, cx: &Cx) -> String {
        match self.0 {
            ChartError::InvalidDate => t!(draw_error_invalid_date).resolve(cx).into_owned(),
            ChartError::YearOutOfRange => t!(draw_error_year_range).resolve(cx).into_owned(),
        }
    }
}

/// A random card on `date`: a random hour (0–23) and gender. Each of the 48
/// cards of a date is equally likely.
pub fn roll(rng: &mut impl Rng, date: BirthDate) -> Result<(CardCode, Chart), BadDate> {
    let birth = BirthInput {
        year: date.year,
        month: date.month,
        day: date.day,
        hour: rng.random_range(0..=23),
        gender: if rng.random_bool(0.5) {
            Gender::Nam
        } else {
            Gender::Nu
        },
    };
    let chart = build_chart(birth).map_err(BadDate)?;
    Ok((CardCode(birth), chart))
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
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    const LAN: BirthDate = BirthDate {
        day: 15,
        month: 4,
        year: SolarYear(1996),
    };

    #[test]
    fn a_draw_keeps_the_date_and_builds_the_chart() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..500 {
            let (CardCode(birth), chart) = roll(&mut rng, LAN).unwrap();
            assert_eq!(BirthDate::of(birth), LAN);
            assert!(birth.hour <= 23);
            assert_eq!(chart, build_chart(birth).unwrap());
        }
    }

    #[test]
    fn draws_cover_both_genders_and_every_hour() {
        let mut rng = StdRng::seed_from_u64(11);
        let draws: Vec<_> = (0..2000)
            .map(|_| roll(&mut rng, LAN).unwrap().0.0)
            .collect();
        assert!(draws.iter().any(|birth| birth.gender == Gender::Nam));
        assert!(draws.iter().any(|birth| birth.gender == Gender::Nu));
        for hour in 0..=23 {
            assert!(draws.iter().any(|birth| birth.hour == hour));
        }
    }

    #[test]
    fn bad_dates_are_errors() {
        let mut rng = StdRng::seed_from_u64(1);
        let feb_30 = BirthDate {
            day: 30,
            month: 2,
            year: SolarYear(2000),
        };
        assert_eq!(
            roll(&mut rng, feb_30).unwrap_err(),
            BadDate(ChartError::InvalidDate)
        );
        let too_late = BirthDate {
            day: 1,
            month: 1,
            year: SolarYear(2300),
        };
        assert_eq!(
            roll(&mut rng, too_late).unwrap_err(),
            BadDate(ChartError::YearOutOfRange)
        );
    }

    #[test]
    fn name_uses_gender_and_year_can_chi() {
        let chart = build_chart(BirthInput {
            year: SolarYear(1996),
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nu,
        })
        .unwrap();
        assert_eq!(card_name(&chart, Lang::Vi), "Nữ mạng Bính Tý");
        assert_eq!(card_name(&chart, Lang::En), "Female · Bính Tý year");
    }
}
