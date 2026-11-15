//! Gacha: the user picks a date, one tap draws a Bản mệnh card on it.
//!
//! - `roll`: a random hour and gender on the picked date, the chart and
//!   the card name
//! - `code`: the birth moment in the card URL
//! - `screen`: the home page draw screen, and the redraw button of the deck
//! - `wheel`: the scroll-snap date wheels
//! - `loader`: the shuffle animation between the tap and the deck
//! - `gacha.js`: the wheel scroll, the date check, and the loader before
//!   the POST
//!
//! A draw is a plain POST to `/la-bai` with the date. The server picks the
//! hour and gender and redirects to `/la-bai/{code}`, so every card is a
//! correct chart for a date on the calendar. Nothing is stored: the URL is
//! the card.

mod code;
mod loader;
mod roll;
mod screen;
mod wheel;

pub use code::CardCode;
pub use roll::{BirthDate, card_name, roll};
pub use screen::{gacha_screen, redraw_button};
