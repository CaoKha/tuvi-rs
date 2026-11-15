//! What the deck reads from the chart: the chính tinh of cung Mệnh, their
//! destiny entry, and the ngũ hành of Bản mệnh, which gives the lucky color.

use tuvi_core::Chart;

use super::content::DESTINY;
use crate::{features::charts::menh_chinh_tinh, i18n::Lang};

/// Ngũ hành in the sinh order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum NguHanh {
    Tho,
    Kim,
    Thuy,
    Moc,
    Hoa,
}

impl NguHanh {
    /// From a name that `tuvi_core` writes, such as "Thổ".
    fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "Thổ" => Self::Tho,
            "Kim" => Self::Kim,
            "Thủy" => Self::Thuy,
            "Mộc" => Self::Moc,
            "Hỏa" => Self::Hoa,
            _ => return None,
        })
    }

    /// The suffix of its CSS color variables: `--element-{css}`.
    pub(super) fn css(self) -> &'static str {
        ["tho", "kim", "thuy", "moc", "hoa"][self as usize]
    }

    /// The lucky color of the element, from `luck-colour.ts` of the old app.
    pub(super) fn luck_color(self, lang: Lang) -> &'static str {
        let (en, vi) = [
            ("Earth brown", "Nâu đất"),
            ("Gold", "Vàng kim"),
            ("Blue", "Xanh lam"),
            ("Green", "Xanh lục"),
            ("Coral red", "Đỏ san hô"),
        ][self as usize];
        lang.pick(en, vi)
    }
}

/// Everything the deck shows about one chart.
pub(super) struct Facts<'a> {
    /// The chính tinh of cung Mệnh. Empty for vô chính diệu.
    pub chinh_tinh: &'a [&'static str],
    /// The index of the destiny entry in [`DESTINY`].
    destiny: usize,
    /// The ngũ hành of Bản mệnh.
    pub ban_menh_hanh: NguHanh,
}

impl<'a> Facts<'a> {
    /// `None` only when the chart holds a name this module does not know.
    pub(super) fn of(chart: &'a Chart) -> Option<Self> {
        let chinh_tinh = menh_chinh_tinh(chart);
        // Bản mệnh ends with its ngũ hành ("Lộ Bàng Thổ").
        let ban_menh_hanh = NguHanh::from_name(chart.ban_menh.rsplit(' ').next()?)?;
        Some(Self {
            chinh_tinh,
            destiny: destiny(chinh_tinh),
            ban_menh_hanh,
        })
    }

    /// The destiny card: `(archetype, mantra)` in `lang`.
    pub(super) fn destiny(&self, lang: Lang) -> (&'static str, &'static str) {
        let (_, archetype_en, mantra_en, archetype_vi, mantra_vi) = DESTINY[self.destiny];
        lang.pick((archetype_en, mantra_en), (archetype_vi, mantra_vi))
    }
}

/// The destiny entry of a set of chính tinh, in any order. An unknown set
/// gets the vô chính diệu entry, so the card never shows blank.
fn destiny(chinh_tinh: &[&str]) -> usize {
    DESTINY
        .iter()
        .position(|(stars, ..)| {
            stars.len() == chinh_tinh.len() && stars.iter().all(|sao| chinh_tinh.contains(sao))
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use tuvi_core::{BirthInput, Gender, SolarYear, build_chart};

    use super::*;

    #[test]
    fn destiny_ignores_star_order() {
        let archetype = |stars: &[&str]| DESTINY[destiny(stars)].1;
        assert_eq!(archetype(&["Tử Vi", "Thiên Phủ"]), "The Monarch");
        assert_eq!(archetype(&["Thiên Phủ", "Tử Vi"]), "The Monarch");
        assert_eq!(archetype(&[]), "The Self-Shaper");
        assert_eq!(archetype(&["Tử Vi", "Cự Môn"]), "The Self-Shaper");
    }

    /// Charts over one 60-year cycle (4 dates a year, every second hour)
    /// all give facts and a known destiny.
    #[test]
    fn every_chart_has_all_facts() {
        for year in 1950..2010 {
            for month in [1, 4, 7, 11] {
                for hour in (0..24).step_by(2) {
                    let birth = BirthInput {
                        year: SolarYear(year),
                        month,
                        day: 15,
                        hour,
                        gender: Gender::Nu,
                    };
                    let chart = build_chart(birth).unwrap();
                    let facts = Facts::of(&chart).unwrap();
                    assert!(facts.chinh_tinh.is_empty() || facts.destiny != 0);
                }
            }
        }
    }
}
