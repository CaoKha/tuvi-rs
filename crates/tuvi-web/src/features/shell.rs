//! The app shell around every page: the logo and the name in the header,
//! and the night and day theme switch.

mod logo;
mod theme;

pub use logo::{LOGO, brand};
pub use theme::{COOKIE as THEME_COOKIE, Theme, theme_toggle};
