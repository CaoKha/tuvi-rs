//! `/card/{code}/chart`: the 12 cung of one card.

use topcoat::{
    Result,
    context::Cx,
    router::{href, page},
    view::{View, view},
};

// `href!` needs the path parameter in scope.
use super::{card_chart, code};
use crate::features::{
    charts::{accent_style, chart_grid},
    gacha::card_name,
};
use crate::{i18n::Lang, t};

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (card, chart) = card_chart(cx)?;
    let name = card_name(&chart, Lang::of(cx));
    Ok(view! {
        <main class="mx-auto max-w-3xl px-3.5 pb-16" style=(accent_style(&chart))>
            <a
                href=(href!(super::show, code: card))
                class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
            >
                "← " (t!(chart_back))
            </a>
            <div class="mt-3">chart_grid(name: name.as_str(), chart: &chart)</div>
        </main>
    })
}
