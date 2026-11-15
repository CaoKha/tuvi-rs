//! `/la-bai/{code}/la-so`: the 12 cung of one card.

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

#[page]
pub async fn show(cx: &Cx) -> Result<impl View> {
    let (card, chart) = card_chart(cx)?;
    let name = card_name(&chart);
    Ok(view! {
        <main class="mx-auto max-w-3xl px-3.5 pb-16" style=(accent_style(&chart))>
            <a
                href=(href!(super::show, code: card))
                class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
            >
                "← Lá bài bản mệnh"
            </a>
            <div class="mt-3">chart_grid(name: name.as_str(), chart: &chart)</div>
        </main>
    })
}
