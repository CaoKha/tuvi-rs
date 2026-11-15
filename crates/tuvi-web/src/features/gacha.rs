//! Gacha: one tap draws a random Bản mệnh card.
//!
//! - `roll`: a random real birth moment, its chart and its card name
//! - `code`: the birth moment in the card URL
//! - `screen`: the home page draw screen, and the draw button that the
//!   Bản mệnh deck also shows
//! - `loader`: the shuffle animation between the tap and the deck
//! - `gacha.js`: holds the POST until the loader has played
//!
//! A draw is a plain POST to `/la-bai`. The server picks the birth moment
//! and redirects to `/la-bai/{code}`, so every card is a correct chart for a
//! date on the calendar. Nothing is stored: the URL is the card.

mod code;
mod loader;
mod roll;
mod screen;

pub use code::CardCode;
pub use roll::{card_name, roll};
pub use screen::{draw_button, gacha_screen};
