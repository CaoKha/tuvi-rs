//! Cast: the home page form that turns a birth moment into a Bản mệnh card.
//!
//! - `birth`: the form fields, their checks, and the card name
//! - `code`: the birth moment in the card URL
//! - `wheel`, `daypart`, `clock`, `segment`: the date wheels, the AM/PM
//!   sun and moon, the birth clock, and the gender segment
//! - `preview`: the shard that shows the constellation of cung Mệnh (or the
//!   sky of vô chính diệu) once date and hour are picked
//! - `loader`: the casting animation between the tap and the card
//! - `screen`: the form put together
//! - `cast.js`: what runtime expressions cannot do: wheel scroll, clock
//!   drag, the random roll, the toasts and the loader delay
//!
//! The form is a plain POST to `/card`. The server checks every field and
//! redirects to `/card/{code}`. Nothing is stored: the URL is the card.

mod birth;
mod clock;
mod code;
mod daypart;
mod loader;
mod preview;
mod screen;
mod segment;
mod wheel;

pub use birth::{BirthForm, card_name};
pub use code::CardCode;
pub use preview::menh_preview;
pub use screen::cast_screen;
