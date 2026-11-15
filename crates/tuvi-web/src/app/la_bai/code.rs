//! `/la-bai/{code}`: the Bản mệnh deck of one card. A new draw lands here.

use topcoat::{
    Result,
    context::Cx,
    router::{error::not_found, href, module_param, page},
    view::{View, view},
};
use tuvi_core::{Chart, build_chart};

use crate::features::{
    ban_menh::ban_menh_screen,
    charts::accent_style,
    gacha::{CardCode, card_name},
};

module_param!(pub code: CardCode else not_found);

pub mod la_so;

/// The chart of the card in the URL. A code that is not on the calendar,
/// or outside 1800–2199, is a 404.
pub(crate) fn card_chart(cx: &Cx) -> Result<(CardCode, Chart)> {
    let card = *code(cx)?;
    let chart = build_chart(card.0).map_err(|_| not_found())?;
    Ok((card, chart))
}

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (card, chart) = card_chart(cx)?;
    let name = card_name(&chart);
    Ok(view! {
        <main class="mx-auto max-w-5xl pb-16" style=(accent_style(&chart))>
            ban_menh_screen(
                name: name.as_str(),
                chart: &chart,
                la_so: href!(la_so::show, code: card).resolve(cx)
            )
        </main>
    })
}
