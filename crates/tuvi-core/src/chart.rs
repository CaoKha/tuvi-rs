//! The public chart types and [`build_chart`].

use serde::{Deserialize, Serialize};

use crate::bureau::Cuc;
use crate::calendar::{self, LunarBirth};
use crate::luck_periods;
use crate::main_stars::{self, Sao};
use crate::minor_stars::{self, PhuTinh};
use crate::natal_element;
use crate::palace::{self, CungRole};
use crate::stems_branches::{DiaChi, ThienCan};
use crate::transformations::{self, Hoa};
use crate::voids;
use crate::year::{LunarYear, SolarYear};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Nam,
    Nu,
}

/// Solar (Gregorian) birth moment, local Vietnam time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BirthInput {
    pub year: SolarYear,
    pub month: u8,
    pub day: u8,
    /// 0..=23
    pub hour: u8,
    pub gender: Gender,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChartError {
    /// Date is not a real Gregorian date, or hour > 23.
    InvalidDate,
    /// Year is outside the lunar table (1800..=2199).
    YearOutOfRange,
}

impl std::fmt::Display for ChartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidDate => "invalid date",
            Self::YearOutOfRange => "year out of supported range (1800-2199)",
        })
    }
}

impl std::error::Error for ChartError {}

/// One of the 12 boxes of the chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Cung {
    /// Địa Chi name with accents, e.g. "Tý".
    pub dia_chi: &'static str,
    /// Thiên Can name with accents, e.g. "Giáp".
    pub thien_can: &'static str,
    /// Role name with accents, e.g. "Mệnh", "Tài Bạch".
    pub role: &'static str,
    pub is_than: bool,
    /// Chính tinh names with accents, e.g. ["Tử Vi", "Thiên Phủ"].
    pub chinh_tinh: Vec<&'static str>,
    /// Every other natal star in this cung, Tràng Sinh ring included, e.g.
    /// ["Lộc Tồn", "Tràng Sinh"]. Sorted by the old engine's component id.
    pub phu_tinh: Vec<&'static str>,
    /// Tứ Hóa of the birth year on stars in this cung. Sorted by Hóa id:
    /// Khoa, Kỵ, Lộc, Quyền.
    pub tu_hoa: Vec<TuHoa>,
    pub tuan: bool,
    pub triet: bool,
    /// Đại hạn: the first and last age (tuổi mụ) of the 10-year period in
    /// this cung, e.g. 6 and 15.
    pub dai_han_start: u8,
    pub dai_han_end: u8,
    /// Tiểu hạn: the Địa Chi of the years whose Tiểu hạn falls in this cung,
    /// e.g. "Mão".
    pub tieu_han: &'static str,
}

/// One Hóa of the birth year and the star that takes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TuHoa {
    /// Star name with accents, e.g. "Liêm Trinh".
    pub sao: &'static str,
    /// e.g. "Hóa Lộc".
    pub hoa: &'static str,
}

/// A computed chart. Not stored: apps keep only [`BirthInput`] and
/// rebuild the chart on read (it takes microseconds).
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Chart {
    pub input: BirthInput,
    pub lunar_day: u8,
    pub lunar_month: u8,
    pub lunar_year: LunarYear,
    /// Hour branch, e.g. "Tý".
    pub hour_chi: &'static str,
    /// Year Can Chi, e.g. "Giáp Tý".
    pub nam_can: &'static str,
    pub nam_chi: &'static str,
    /// e.g. "Thủy Nhị cục".
    pub cuc: &'static str,
    /// Bản mệnh (nạp âm) of the lunar year, e.g. "Lộ Bàng Thổ".
    pub ban_menh: &'static str,
    /// Indexed by Địa Chi: 0 = Tý, 1 = Sửu, …, 11 = Hợi.
    pub cungs: [Cung; 12],
}

/// Build the natal chart: lunar date, year Can Chi, Mệnh/Thân, the 12 cung
/// roles and Thiên Can, Cục, Bản mệnh, the 14 chính tinh, the phụ tinh,
/// Tứ Hóa, Tuần/Triệt, Đại hạn and Tiểu hạn.
///
/// Only the non-empty `Vec` fields allocate. Release build: about
/// 0.8 µs per call, allocations included (see `tests/bench.rs`).
pub fn build_chart(input: BirthInput) -> Result<Chart, ChartError> {
    let BirthInput {
        year,
        month,
        day,
        hour,
        ..
    } = input;
    if hour > 23 || day == 0 || day > calendar::days_in_month(year.0, month) {
        return Err(ChartError::InvalidDate);
    }
    if !(calendar::FIRST_YEAR..=calendar::LAST_YEAR).contains(&year.0) {
        return Err(ChartError::YearOutOfRange);
    }

    let birth = LunarBirth::from_solar(year, month, day, hour)?;
    let lunar = birth.date;
    let nam_can = ThienCan::from_index(lunar.year.0 + 6);
    let nam_chi = DiaChi::from_index(lunar.year.0 + 8);

    let menh = palace::menh_position(lunar.month, birth.hour);
    let than = palace::than_position(menh, birth.hour);
    let cuc = Cuc::from_menh(nam_can, menh);
    let tu_vi = main_stars::tu_vi_position(lunar.day, cuc.number());
    let sao_positions = main_stars::chinh_tinh_positions(tu_vi);

    let van = palace::van_direction(input.gender, nam_chi);
    let phu_tinh_positions = minor_stars::positions(&minor_stars::Input {
        nam_can,
        nam_chi,
        lunar_month: lunar.month,
        lunar_day: lunar.day,
        hour: birth.hour,
        menh,
        than,
        cuc,
        van,
    });
    let tuan = voids::tuan(nam_can, nam_chi);
    let triet = voids::triet(nam_can);
    let dai_han_start = luck_periods::dai_han_start(cuc.number(), menh, van);
    let tieu_han = luck_periods::tieu_han(nam_chi, van);

    // Bucket the stars by cung in one pass each. `ALL` is in the old engine's
    // id order, so each bucket is already sorted.
    let mut chinh_tinh = buckets(&sao_positions);
    for sao in Sao::ALL {
        chinh_tinh[sao_positions[sao as usize].index()].push(sao.name());
    }
    let mut phu_tinh = buckets(&phu_tinh_positions);
    for sao in PhuTinh::ALL {
        phu_tinh[phu_tinh_positions[sao as usize].index()].push(sao.name());
    }
    let mut tu_hoa: [Vec<TuHoa>; 12] = Default::default();
    for hoa in Hoa::ALL {
        let (sao, position) = match transformations::target(nam_can, hoa) {
            transformations::Target::Chinh(sao) => (sao.name(), sao_positions[sao as usize]),
            transformations::Target::Phu(sao) => (sao.name(), phu_tinh_positions[sao as usize]),
        };
        tu_hoa[position.index()].push(TuHoa {
            sao,
            hoa: hoa.name(),
        });
    }

    let cungs = std::array::from_fn(|i| {
        let dia_chi = DiaChi::ALL[i];
        Cung {
            dia_chi: dia_chi.name(),
            thien_can: palace::cung_thien_can(nam_can, dia_chi).name(),
            role: CungRole::at(dia_chi, menh).name(),
            is_than: dia_chi == than,
            chinh_tinh: std::mem::take(&mut chinh_tinh[i]),
            phu_tinh: std::mem::take(&mut phu_tinh[i]),
            tu_hoa: std::mem::take(&mut tu_hoa[i]),
            tuan: tuan.contains(&dia_chi),
            triet: triet.contains(&dia_chi),
            dai_han_start: dai_han_start[i],
            dai_han_end: dai_han_start[i] + 9,
            tieu_han: tieu_han[i].name(),
        }
    });

    Ok(Chart {
        input,
        lunar_day: lunar.day,
        lunar_month: lunar.month,
        lunar_year: lunar.year,
        hour_chi: birth.hour.name(),
        nam_can: nam_can.name(),
        nam_chi: nam_chi.name(),
        cuc: cuc.name(),
        ban_menh: natal_element::name(lunar.year),
        cungs,
    })
}

/// 12 empty vectors, each with the capacity for the stars at that Địa Chi.
fn buckets(positions: &[DiaChi]) -> [Vec<&'static str>; 12] {
    let mut counts = [0_usize; 12];
    for position in positions {
        counts[position.index()] += 1;
    }
    counts.map(Vec::with_capacity)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(year: i32, month: u8, day: u8, hour: u8) -> BirthInput {
        BirthInput {
            year: SolarYear(year),
            month,
            day,
            hour,
            gender: Gender::Nam,
        }
    }

    #[test]
    fn invalid_dates() {
        for (y, m, d, h) in [
            (2000, 2, 30, 0),
            (1900, 2, 29, 0),
            (2000, 13, 1, 0),
            (2000, 0, 1, 0),
            (2000, 1, 0, 0),
            (2000, 1, 1, 24),
        ] {
            assert_eq!(build_chart(input(y, m, d, h)), Err(ChartError::InvalidDate));
        }
    }

    #[test]
    fn out_of_range() {
        for (y, m, d, h) in [
            (1799, 6, 1, 0),
            (2200, 6, 1, 0),
            (i32::MIN, 1, 1, 0),
            (i32::MAX, 12, 31, 23),
        ] {
            assert_eq!(
                build_chart(input(y, m, d, h)),
                Err(ChartError::YearOutOfRange)
            );
        }
    }
}
