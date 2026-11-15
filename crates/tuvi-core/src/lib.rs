//! Tử Vi chart engine.
//!
//! Pure computation: no IO, no network, no database. Apps call
//! [`build_chart`] with a [`BirthInput`] and render the returned [`Chart`].
//!
//! Modules, in the order the chart is built:
//! - `calendar`: solar date to lunar date, birth hour branch
//! - `stems_branches`: Thiên Can and Địa Chi
//! - `palace`: the 12 cung roles, Mệnh and Thân
//! - `bureau`: Cục
//! - `main_stars`: the 14 main stars
//! - `minor_stars`: the other natal stars, Tràng Sinh ring included
//! - `transformations`: Tứ Hóa of the birth year
//! - `voids`: Tuần and Triệt
//! - `natal_element`: Bản mệnh of the birth year
//! - `luck_periods`: Đại hạn and Tiểu hạn of each cung
//! - `daily_flow`: one day and its 12 canh giờ over a chart (lưu nhật, lưu thời)
//! - `chart`: the public types and `build_chart`
//! - `year`: `SolarYear` and `LunarYear`

mod bureau;
mod calendar;
mod chart;
mod daily_flow;
mod luck_periods;
mod main_stars;
mod minor_stars;
mod natal_element;
mod palace;
mod stems_branches;
mod transformations;
mod voids;
mod year;

pub use chart::{BirthInput, Chart, ChartError, Cung, Gender, TuHoa, build_chart};
pub use daily_flow::{DayFlow, DayHoa, Gio, day_flow};
pub use year::{LunarYear, SolarYear};
