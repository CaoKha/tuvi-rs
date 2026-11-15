//! Hôm nay: a timed roadmap for today over the card's chart.
//!
//! - `guide`: the texts of `data/daily_guide.json`, in English and Vietnamese
//! - `clock`: today's date and hour in Vietnam
//! - `roadmap`: the theme of the day, its Tứ Hóa and the 12 canh giờ
//!
//! `tuvi_core::day_flow` computes the day; this module only picks the texts.

mod clock;
mod guide;
mod roadmap;

pub use guide::Guide;
pub use roadmap::today_roadmap;
