//! `POST /card`: casts the card of the birth form (`features::cast`).

use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, error::see_other, href, page},
    view::{View, view},
};

use crate::features::cast::{BirthForm, CardCode, cast_screen};

pub mod code;

/// Checks the birth form, then opens the card. On bad input, shows the form
/// again with the sent values and the error.
#[page(POST)]
pub async fn draw(cx: &Cx, Form(form): Form<BirthForm>) -> Result<impl View> {
    let error = match form.validate() {
        Ok(birth) => {
            let card = CardCode(birth);
            return Err(see_other(href!(code::show, code: card).resolve(cx)).into());
        }
        Err(error) => error.message(cx),
    };
    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">
            cast_screen(sent: Some(&form), error: Some(error))
        </main>
    })
}
