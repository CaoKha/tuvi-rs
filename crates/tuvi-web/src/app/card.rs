//! `POST /card`: draws a card on the picked date (`features::gacha`).

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::features::gacha::{BirthDate, gacha_screen, roll};

pub mod code;

/// Draws a random hour and gender on the posted date, then opens the
/// Bản mệnh deck of that card. A date that is not on the calendar shows the
/// draw screen again with the date and the error.
#[page(POST)]
pub async fn draw(cx: &Cx, Form(date): Form<BirthDate>) -> Result<impl View> {
    let error = match roll(&mut rand::rng(), date) {
        Ok((card, _)) => {
            return Err(see_other(href!(code::show, code: card).resolve(cx)).into());
        }
        Err(error) => error.message(cx),
    };
    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">
            gacha_screen(date: Some(date), error: Some(error))
        </main>
    })
}
