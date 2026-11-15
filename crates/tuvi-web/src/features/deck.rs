//! The Bản mệnh screen: Lá Bài Bản Mệnh next to today's roadmap. A new card
//! lands here.
//!
//! - `constellation`: the constellation of each chính tinh over its ghost
//!   portrait, and the scattered sky
//! - `content`: the destiny text, in English and Vietnamese
//! - `facts`: what the deck reads from the chart
//! - `destiny`: Lá Bài Bản Mệnh
//! - `screen`: the header, the card and the roadmap (`features::today`)

mod constellation;
mod content;
mod destiny;
mod facts;
mod screen;

pub use constellation::{chom_figure, chom_sao_of, sky};
pub use screen::deck_screen;
