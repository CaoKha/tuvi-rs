//! Hôm nay: a timed roadmap of one day over the card's chart: today, or
//! the day picked in the calendar.
//!
//! - `guide`: the texts of `data/daily_guide.json`, in English and Vietnamese
//! - `clock`: today's date and hour in Vietnam, and date helpers
//! - `roadmap`: the theme of the day, its Tứ Hóa, the 12 canh giờ and the
//!   calendar
//!
//! `tuvi_core::day_flow` computes the day; this module only picks the texts.

mod clock;
mod guide;
mod roadmap;
mod zone;

pub use guide::Guide;
pub use roadmap::day_roadmap;
