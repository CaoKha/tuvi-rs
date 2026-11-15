//! The draw: a random birth moment that the engine accepts.

use rand::Rng;
use tuvi_core::{BirthInput, Chart, Gender, build_chart};

use super::CardCode;

/// The years a draw picks from. The engine supports 1800–2199; draws keep
/// to the years of people alive now and soon.
const YEARS: std::ops::RangeInclusive<i32> = 1900..=2099;

/// Draws before giving up. A random day 1–31 is a real date about 98% of
/// the time, so 64 tries never all fail in practice.
const MAX_TRIES: usize = 64;

/// A random card: its code and its chart. Each real date, hour and gender in
/// [`YEARS`] is equally likely: a draw that is not on the calendar (such as
/// 30/02) is thrown away and drawn again.
///
/// Returns `None` only if every try fails, which does not happen in practice.
pub fn roll(rng: &mut impl Rng) -> Option<(CardCode, Chart)> {
    (0..MAX_TRIES).find_map(|_| {
        let birth = BirthInput {
            year: rng.random_range(YEARS),
            month: rng.random_range(1..=12),
            day: rng.random_range(1..=31),
            hour: rng.random_range(0..=23),
            gender: if rng.random_bool(0.5) {
                Gender::Nam
            } else {
                Gender::Nu
            },
        };
        build_chart(birth)
            .ok()
            .map(|chart| (CardCode(birth), chart))
    })
}

/// The card's name, as Tử Vi books name a chart: gender and the Can Chi of
/// the lunar year, for example "Nữ mạng Bính Tý".
pub fn card_name(chart: &Chart) -> String {
    let gender = match chart.input.gender {
        Gender::Nam => "Nam",
        Gender::Nu => "Nữ",
    };
    format!("{gender} mạng {} {}", chart.nam_can, chart.nam_chi)
}

#[cfg(test)]
mod tests {
    use rand::{SeedableRng, rngs::StdRng};

    use super::*;

    #[test]
    fn every_draw_is_a_real_chart_in_range() {
        let mut rng = StdRng::seed_from_u64(7);
        for _ in 0..2000 {
            let (CardCode(birth), chart) = roll(&mut rng).unwrap();
            assert!(YEARS.contains(&birth.year));
            assert!(birth.hour <= 23);
            assert_eq!(chart, build_chart(birth).unwrap());
        }
    }

    #[test]
    fn draws_cover_both_genders_and_every_hour() {
        let mut rng = StdRng::seed_from_u64(11);
        let draws: Vec<_> = (0..2000).map(|_| roll(&mut rng).unwrap().0.0).collect();
        assert!(draws.iter().any(|birth| birth.gender == Gender::Nam));
        assert!(draws.iter().any(|birth| birth.gender == Gender::Nu));
        for hour in 0..=23 {
            assert!(draws.iter().any(|birth| birth.hour == hour));
        }
    }

    #[test]
    fn name_uses_gender_and_year_can_chi() {
        let chart = build_chart(BirthInput {
            year: 1996,
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nu,
        })
        .unwrap();
        assert_eq!(card_name(&chart), "Nữ mạng Bính Tý");
    }
}
