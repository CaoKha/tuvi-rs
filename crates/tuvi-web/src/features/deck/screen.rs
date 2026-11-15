//! The Bản mệnh screen: the header, Lá Bài Bản Mệnh and today's roadmap.
//! On a phone they stack. From `lg` the card is on the left (364px) and the
//! roadmap on the right.
//!
//! The header names the birth moment of the card, solar and lunar, so the
//! user sees which day the sky belongs to.

use topcoat::{
    Result,
    context::Cx,
    router::error::not_found,
    view::{View, view},
};
use tuvi_core::Chart;

use super::{destiny::destiny_card, facts::Facts};
use crate::{
    features::{
        gacha::{BirthDate, card_name, redraw_button},
        today::today_roadmap,
    },
    i18n::Lang,
    t,
};

#[topcoat::view::component]
pub async fn deck_screen(cx: &Cx, chart: &Chart, chart_url: String) -> Result<impl View> {
    let Some(facts) = Facts::of(chart) else {
        return Err(not_found().into());
    };
    let lang = Lang::of(cx);
    let lunar = format!("{}/{}", chart.lunar_day, chart.lunar_month);
    let hour = format!("{:02}:00", chart.input.hour);
    Ok(view! {
        <header class="px-4">
            <div class="flex items-center justify-between gap-3.5">
                <p class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase">
                    (card_name(chart, lang))
                </p>
                <p
                    class="flex items-center gap-2 rounded-full border border-accent-glow bg-accent/12 px-3.5 py-1.5 text-[13px] font-semibold"
                >
                    <span
                        class="size-2.25 rounded-full bg-linear-135 from-accent to-accent-2 shadow-[0_0_10px_var(--accent-glow)]"
                    ></span>
                    if facts.chinh_tinh.is_empty() {
                        <span class="text-accent">"Vô Chính Diệu"</span>
                    }
                    for (index, sao) in facts.chinh_tinh.iter().enumerate() {
                        if index > 0 {
                            <span class="text-muted">"·"</span>
                        }
                        <span class=(if index == 0 { "text-accent" } else { "text-accent-2" })>
                            (*sao)
                        </span>
                    }
                </p>
            </div>
            <h1 class="mt-2.5 text-[26px] leading-[1.06] font-semibold tabular-nums">
                (solar_date(chart))
            </h1>
            <p class="mt-1.75 text-[13.5px] text-muted">
                (t!(
                    deck_birth_moment,
                    hour: hour.as_str(),
                    chi: chart.hour_chi,
                    lunar: lunar.as_str(),
                    year_can: chart.nam_can,
                    year_chi: chart.nam_chi
                ))
            </p>
            <p
                class="mt-2.5 inline-flex items-center gap-2 rounded-full border border-glass-line bg-glass px-3 py-1 text-[12px]"
                style=(format!("--luck: var(--element-{})", facts.ban_menh_hanh.css()))
            >
                <span class="size-2.5 rounded-full bg-(--luck) shadow-[0_0_8px_var(--luck)]"></span>
                (t!(deck_lucky_color))
                <b class="font-semibold">(facts.ban_menh_hanh.luck_color(lang))</b>
            </p>
        </header>
        <div
            class="grid justify-items-center gap-5 px-4 py-8 lg:grid-cols-[364px_minmax(0,1fr)] lg:items-start lg:justify-items-stretch"
        >
            <div class="lg:sticky lg:top-6">destiny_card(facts: &facts)</div>
            <div class="w-full min-w-0">today_roadmap(chart: chart)</div>
        </div>
        <div class="mx-auto flex max-w-sm flex-col gap-3 px-4">
            redraw_button(date: BirthDate::of(chart.input))
            <a
                href=(chart_url)
                class="pill-ghost flex h-12 items-center justify-center rounded-full text-sm font-semibold"
            >
                (t!(deck_full_chart))
            </a>
        </div>
    })
}

/// The solar birth date, for example "15/04/1996".
fn solar_date(chart: &Chart) -> String {
    let birth = chart.input;
    format!("{:02}/{:02}/{}", birth.day, birth.month, birth.year)
}

#[cfg(test)]
mod tests {
    use tuvi_core::{BirthInput, Gender, SolarYear, build_chart};

    use super::*;

    #[test]
    fn header_shows_the_solar_date() {
        let chart = build_chart(BirthInput {
            year: SolarYear(1996),
            month: 4,
            day: 5,
            hour: 7,
            gender: Gender::Nam,
        })
        .unwrap();
        assert_eq!(solar_date(&chart), "05/04/1996");
    }
}
