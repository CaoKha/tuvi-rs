//! Lưu nhật and lưu thời: how one day and its 12 canh giờ move over a natal
//! chart.
//!
//! - Đẩu Quân: from the cung of the viewed lunar year's Địa Chi (lưu Thái
//!   Tuế), count back (nghịch) to the birth month, then forward (thuận) to
//!   the birth hour. It is the lưu nguyệt Mệnh of tháng Giêng.
//! - Lưu nguyệt Mệnh: Đẩu Quân, then one cung forward per lunar month.
//! - Lưu nhật Mệnh: mồng 1 sits on the lưu nguyệt Mệnh, then one cung
//!   forward per lunar day.
//! - A leap month counts as the month it repeats, as everywhere in the
//!   engine.
//! - Tứ Hóa of the day: the Thiên Can of the day, with the birth-year table.
//! - Lưu thời: giờ Tý starts at the lưu nhật Mệnh cung, then one cung per
//!   canh giờ, clockwise.
//! - Giờ hoàng đạo: the 12 hour gods. Thanh Long starts at Thân on Tý and
//!   Ngọ days, two hours later for each next pair of day chi.
//! - The Thiên Can of each hour: Ngũ Thử Độn (Giáp and Kỷ days start at
//!   Giáp Tý).

use crate::calendar::{self, LunarBirth, jdn};
use crate::chart::{Chart, ChartError};
use crate::stems_branches::{DiaChi, ThienCan};
use crate::transformations::{self, Hoa};
use crate::year::{LunarYear, SolarYear};

/// The 12 hour gods in order, and whether each one is hoàng đạo.
const GODS: [(&str, bool); 12] = [
    ("Thanh Long", true),
    ("Minh Đường", true),
    ("Thiên Hình", false),
    ("Chu Tước", false),
    ("Kim Quỹ", true),
    ("Bảo Quang", true),
    ("Bạch Hổ", false),
    ("Ngọc Đường", true),
    ("Thiên Lao", false),
    ("Huyền Vũ", false),
    ("Tư Mệnh", true),
    ("Câu Trận", false),
];

/// The Tứ Hóa of the day, in the usual order.
const HOA_ORDER: [Hoa; 4] = [Hoa::Loc, Hoa::Quyen, Hoa::Khoa, Hoa::Ky];

/// One Hóa of the day: its star and the natal cung that holds the star.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayHoa {
    /// e.g. "Hóa Lộc".
    pub hoa: &'static str,
    /// e.g. "Thiên Đồng".
    pub sao: &'static str,
    /// The role of the cung with the star, e.g. "Tài Bạch".
    pub role: &'static str,
}

/// One canh giờ of the day.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gio {
    /// e.g. "Thìn".
    pub chi: &'static str,
    /// The Thiên Can of the hour, e.g. "Mậu".
    pub can: &'static str,
    /// The first clock hour: 23 for Tý, then 1, 3, …, 21.
    pub start_hour: u8,
    /// The role of the natal cung that the hour activates (lưu thời).
    pub role: &'static str,
    /// The hour god, e.g. "Kim Quỹ".
    pub god: &'static str,
    pub hoang_dao: bool,
    /// The Hóa of the day whose star sits in `role`, e.g. ["Hóa Lộc"].
    pub tu_hoa: Vec<&'static str>,
}

/// One solar day over a natal chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayFlow {
    pub year: SolarYear,
    pub month: u8,
    pub day: u8,
    pub lunar_day: u8,
    pub lunar_month: u8,
    pub lunar_year: LunarYear,
    /// Can Chi of the day, e.g. "Giáp" and "Tý".
    pub ngay_can: &'static str,
    pub ngay_chi: &'static str,
    /// The role of the lưu nguyệt Mệnh cung: the theme of the lunar month.
    pub month_role: &'static str,
    /// The role of the lưu nhật Mệnh cung: the theme of the day.
    pub menh_role: &'static str,
    /// Lộc, Quyền, Khoa, Kỵ of the day.
    pub tu_hoa: [DayHoa; 4],
    /// The 12 canh giờ, Tý first.
    pub gio: [Gio; 12],
}

/// Đẩu Quân of a year: "Lấy Thái Tuế gọi tháng Giêng, tháng nghịch, giờ
/// thuận". The Thái Tuế cung is tháng Giêng; count back to the birth month;
/// that cung is giờ Tý; count forward to the birth hour (tuvisonkhiem.vn,
/// sao Đẩu Quân).
fn dau_quan(thai_tue: DiaChi, birth_month: u8, birth_hour: DiaChi) -> DiaChi {
    thai_tue
        .add(1 - i32::from(birth_month))
        .add(birth_hour.index() as i32)
}

/// The role of the natal cung that holds `sao`, if the chart has it.
fn role_of(chart: &Chart, sao: &str) -> &'static str {
    chart
        .cungs
        .iter()
        .find(|cung| cung.chinh_tinh.contains(&sao) || cung.phu_tinh.contains(&sao))
        .map_or("", |cung| cung.role)
}

/// How the solar day `year-month-day` moves over `chart`.
///
/// Returns an error for a date that is not on the calendar or outside the
/// lunar table.
pub fn day_flow(chart: &Chart, year: SolarYear, month: u8, day: u8) -> Result<DayFlow, ChartError> {
    if day == 0 || day > calendar::days_in_month(year.0, month) {
        return Err(ChartError::InvalidDate);
    }
    let lunar = calendar::solar_to_lunar(year, month, day)?;
    let jd = jdn(i32::from(day), i32::from(month), year.0);
    let ngay_can = ThienCan::from_index(jd + 9);
    let ngay_chi = DiaChi::from_index(jd + 1);

    // Đẩu Quân, lưu nguyệt and lưu nhật Mệnh.
    let input = chart.input;
    let birth = LunarBirth::from_solar(input.year, input.month, input.day, input.hour)?;
    let thai_tue = DiaChi::from_index(lunar.year.0 + 8);
    let dau_quan = dau_quan(thai_tue, birth.date.month, birth.hour);
    let luu_nguyet = dau_quan.add(i32::from(lunar.month) - 1);
    let luu_nhat = luu_nguyet.add(i32::from(lunar.day) - 1);

    let tu_hoa = HOA_ORDER.map(|hoa| {
        let sao = match transformations::target(ngay_can, hoa) {
            transformations::Target::Chinh(sao) => sao.name(),
            transformations::Target::Phu(sao) => sao.name(),
        };
        DayHoa {
            hoa: hoa.name(),
            sao,
            role: role_of(chart, sao),
        }
    });

    // Thanh Long: Thân on Tý/Ngọ days, then two hours later per chi pair.
    let thanh_long = 8 + 2 * (ngay_chi.index() % 6) as i32;
    // Ngũ Thử Độn: the Thiên Can of giờ Tý.
    let can_ty = 2 * (ngay_can.index() % 5) as i32;
    let gio = std::array::from_fn(|index| {
        let chi = DiaChi::from_index(index as i32);
        let palace = luu_nhat.add(index as i32);
        let role = chart.cungs[palace.index()].role;
        let (god, hoang_dao) = GODS[(index as i32 - thanh_long).rem_euclid(12) as usize];
        Gio {
            chi: chi.name(),
            can: ThienCan::from_index(can_ty + index as i32).name(),
            start_hour: ((index * 2 + 23) % 24) as u8,
            role,
            god,
            hoang_dao,
            tu_hoa: tu_hoa
                .iter()
                .filter(|hoa| hoa.role == role)
                .map(|hoa| hoa.hoa)
                .collect(),
        }
    });

    Ok(DayFlow {
        year,
        month,
        day,
        lunar_day: lunar.day,
        lunar_month: lunar.month,
        lunar_year: lunar.year,
        ngay_can: ngay_can.name(),
        ngay_chi: ngay_chi.name(),
        month_role: chart.cungs[luu_nguyet.index()].role,
        menh_role: chart.cungs[luu_nhat.index()].role,
        tu_hoa,
        gio,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::{BirthInput, Gender, build_chart};

    fn chart() -> Chart {
        build_chart(BirthInput {
            year: SolarYear(1996),
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nu,
        })
        .unwrap()
    }

    #[test]
    fn day_can_chi_matches_the_calendar() {
        // 01/01/2000 is a Mậu Ngọ day. 15/11/2026 is a Quý Tị day, 7/10
        // âm lịch (checked against xemlicham.com and baohatinh.vn).
        let flow = day_flow(&chart(), SolarYear(2000), 1, 1).unwrap();
        assert_eq!((flow.ngay_can, flow.ngay_chi), ("Mậu", "Ngọ"));
        let flow = day_flow(&chart(), SolarYear(2026), 11, 15).unwrap();
        assert_eq!((flow.ngay_can, flow.ngay_chi), ("Quý", "Tị"));
        assert_eq!((flow.lunar_day, flow.lunar_month), (7, 10));
    }

    #[test]
    fn menh_follows_dau_quan() {
        // Born 15/04/1996 at 22h: lunar month 2, giờ Hợi.
        let chart = chart();
        // 15/11/2026 is 7/10 of Bính Ngọ: Thái Tuế at Ngọ, back 1 month to
        // Tị, forward 11 hours to Thìn (Đẩu Quân). Month 10 is Sửu, day 7
        // is Mùi.
        let flow = day_flow(&chart, SolarYear(2026), 11, 15).unwrap();
        assert_eq!(flow.month_role, chart.cungs[1].role);
        assert_eq!(flow.menh_role, chart.cungs[7].role);
        // 01/01/2000 is 25/11 of Kỷ Mão: Thái Tuế at Mão, Đẩu Quân at Sửu,
        // month 11 at Hợi, day 25 at Hợi.
        let flow = day_flow(&chart, SolarYear(2000), 1, 1).unwrap();
        assert_eq!((flow.lunar_day, flow.lunar_month), (25, 11));
        assert_eq!(flow.month_role, chart.cungs[11].role);
        assert_eq!(flow.menh_role, chart.cungs[11].role);
    }

    #[test]
    fn dau_quan_matches_the_sources() {
        // tuvisonkhiem.vn: a Tý year, tháng 3, giờ Dần gives Đẩu Quân at Tý.
        assert_eq!(dau_quan(DiaChi::Ty, 3, DiaChi::Dan), DiaChi::Ty);
        // xemvanmenh: Thái Tuế at Dần, tháng 8, giờ Mùi gives Dần.
        assert_eq!(dau_quan(DiaChi::Dan, 8, DiaChi::Mui), DiaChi::Dan);
    }

    #[test]
    fn the_next_day_moves_one_cung() {
        let chart = chart();
        let today = day_flow(&chart, SolarYear(2026), 11, 15).unwrap();
        let tomorrow = day_flow(&chart, SolarYear(2026), 11, 16).unwrap();
        let index = |role| {
            chart
                .cungs
                .iter()
                .position(|cung| cung.role == role)
                .unwrap()
        };
        assert_eq!(index(tomorrow.menh_role), (index(today.menh_role) + 1) % 12);
    }

    #[test]
    fn hours_start_at_the_luu_nhat_menh() {
        let chart = chart();
        let flow = day_flow(&chart, SolarYear(2026), 11, 15).unwrap();
        // Lưu nhật Mệnh at Mùi: giờ Tý there, giờ Sửu at Thân.
        assert_eq!(flow.gio[0].role, chart.cungs[7].role);
        assert_eq!(flow.gio[1].role, chart.cungs[8].role);
        assert_eq!(flow.gio[6].role, chart.cungs[1].role);
    }

    #[test]
    fn hour_gods_start_by_the_day_chi() {
        let chart = chart();
        // Ngọ day: Thanh Long at Thân.
        let flow = day_flow(&chart, SolarYear(2000), 1, 1).unwrap();
        assert_eq!(flow.gio[8].god, "Thanh Long");
        assert_eq!(flow.gio[9].god, "Minh Đường");
        assert_eq!(flow.gio[0].god, "Kim Quỹ");
        // Tị day 15/11/2026: the almanacs list Sửu, Thìn, Ngọ, Mùi, Tuất
        // and Hợi as hoàng đạo.
        let flow = day_flow(&chart, SolarYear(2026), 11, 15).unwrap();
        let hoang_dao: Vec<_> = flow
            .gio
            .iter()
            .filter(|gio| gio.hoang_dao)
            .map(|gio| gio.chi)
            .collect();
        assert_eq!(hoang_dao, ["Sửu", "Thìn", "Ngọ", "Mùi", "Tuất", "Hợi"]);
    }

    #[test]
    fn hour_stems_follow_ngu_thu_don() {
        // Mậu day: giờ Tý is Nhâm Tý, giờ Thìn is Bính Thìn.
        let flow = day_flow(&chart(), SolarYear(2000), 1, 1).unwrap();
        assert_eq!((flow.gio[0].can, flow.gio[0].chi), ("Nhâm", "Tý"));
        assert_eq!((flow.gio[4].can, flow.gio[4].chi), ("Bính", "Thìn"));
        assert_eq!(flow.gio[0].start_hour, 23);
        assert_eq!(flow.gio[4].start_hour, 7);
    }

    #[test]
    fn tu_hoa_follows_the_day_can() {
        let chart = chart();
        // Mậu: Tham Lang, Thái Âm, Hữu Bật, Thiên Cơ.
        let flow = day_flow(&chart, SolarYear(2000), 1, 1).unwrap();
        let stars: Vec<_> = flow.tu_hoa.iter().map(|hoa| (hoa.hoa, hoa.sao)).collect();
        assert_eq!(
            stars,
            [
                ("Hóa Lộc", "Tham Lang"),
                ("Hóa Quyền", "Thái Âm"),
                ("Hóa Khoa", "Hữu Bật"),
                ("Hóa Kỵ", "Thiên Cơ"),
            ]
        );
        for hoa in flow.tu_hoa {
            assert!(!hoa.role.is_empty(), "{} has no cung", hoa.sao);
            let gio = flow.gio.iter().find(|gio| gio.role == hoa.role).unwrap();
            assert!(gio.tu_hoa.contains(&hoa.hoa));
        }
    }

    #[test]
    fn bad_dates_are_errors() {
        assert_eq!(
            day_flow(&chart(), SolarYear(2000), 2, 30),
            Err(ChartError::InvalidDate)
        );
        assert_eq!(
            day_flow(&chart(), SolarYear(2300), 1, 1),
            Err(ChartError::YearOutOfRange)
        );
    }
}
