//! The chart of a card.
//!
//! - `grid`: the chart drawing and the cung popups
//! - `accent`: the chart's accent color from cung Mệnh
//!
//! Nothing is stored. Pages build the chart from the card code in the URL
//! with `tuvi_core` on each request; this takes about a microsecond.

mod accent;
mod grid;

pub use accent::{accent_style, menh_chinh_tinh};
pub use grid::chart_grid;
