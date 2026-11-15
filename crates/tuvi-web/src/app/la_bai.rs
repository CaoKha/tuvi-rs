//! `POST /la-bai`: draws a random card (`features::gacha`).

use topcoat::{
    Result,
    context::Cx,
    router::{
        error::{SeeOther, see_other},
        href, route,
    },
};

use crate::features::gacha::roll;

pub mod code;

/// Draws a random birth moment, then opens the Bản mệnh deck of that card.
#[route(POST)]
pub async fn draw(cx: &Cx) -> Result<SeeOther> {
    let Some((card, _)) = roll(&mut rand::rng()) else {
        // Every draw failed: this does not happen in practice. Back to the
        // home page, where the user can tap again.
        return Ok(see_other(href!(crate::app::home).resolve(cx)));
    };
    Ok(see_other(href!(code::show, code: card).resolve(cx)))
}
