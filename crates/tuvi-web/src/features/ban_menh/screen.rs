//! The Bản mệnh screen: the header and the deck. On a phone the deck is a
//! row of cards that scrolls sideways and snaps to each card. From `lg` the
//! main card is on the left and the two other cards are on the right.
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

use super::{advice::advice_card, destiny::destiny_card, facts::Facts, luck::luck_card};
use crate::features::gacha::draw_button;

#[topcoat::view::component]
pub async fn ban_menh_screen(
    cx: &Cx,
    name: &str,
    chart: &Chart,
    la_so: String,
) -> Result<impl View> {
    let Some(facts) = Facts::of(chart) else {
        return Err(not_found().into());
    };
    Ok(view! {
        <header class="px-4">
            <div class="flex items-center justify-between gap-3.5">
                <p
                    class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase"
                >
                    (name)
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
                        <span
                            class=(if index == 0 {
                                "text-accent"
                            } else {
                                "text-accent-2"
                            })
                        >
                            (*sao)
                        </span>
                    }
                </p>
            </div>
            <h1 class="mt-2.5 text-[26px] leading-[1.06] font-semibold tabular-nums">
                (solar_date(chart))
            </h1>
            <p class="mt-1.75 text-[13.5px] text-muted">(birth_moment(chart))</p>
            <p class="mt-1 text-[13.5px] text-muted lg:hidden">
                "Vuốt sang để lật hai lá bài phụ."
            </p>
        </header>
        <div
            class="flex snap-x snap-mandatory items-center gap-3.5 overflow-x-auto px-[max(16px,calc(50%-165px))] py-10 lg:grid lg:grid-cols-[364px_minmax(0,1fr)] lg:items-stretch lg:gap-5 lg:px-4"
        >
            destiny_card(facts: &facts)
            luck_card(facts: &facts)
            advice_card(facts: &facts)
        </div>
        <div class="mx-auto flex max-w-sm flex-col gap-3 px-4">
            draw_button(label: "✦ Rút lá khác", class: "w-full")
            <a
                href=(la_so)
                class="pill-ghost flex h-12 items-center justify-center rounded-full text-sm font-semibold"
            >
                "Xem đủ 12 cung"
            </a>
        </div>
    })
}

/// The solar birth date, for example "15/04/1996".
fn solar_date(chart: &Chart) -> String {
    let birth = chart.input;
    format!("{:02}/{:02}/{}", birth.day, birth.month, birth.year)
}

/// The hour and the lunar date, for example
/// "22:00 · giờ Hợi · âm lịch 28/2 năm Bính Tý".
fn birth_moment(chart: &Chart) -> String {
    format!(
        "{:02}:00 · giờ {} · âm lịch {}/{} năm {} {}",
        chart.input.hour,
        chart.hour_chi,
        chart.lunar_day,
        chart.lunar_month,
        chart.nam_can,
        chart.nam_chi,
    )
}

#[cfg(test)]
mod tests {
    use tuvi_core::{BirthInput, Gender, build_chart};

    use super::*;

    #[test]
    fn header_shows_solar_and_lunar_dates() {
        let chart = build_chart(BirthInput {
            year: 1996,
            month: 4,
            day: 5,
            hour: 7,
            gender: Gender::Nam,
        })
        .unwrap();
        assert_eq!(solar_date(&chart), "05/04/1996");
        assert_eq!(
            birth_moment(&chart),
            format!(
                "07:00 · giờ Thìn · âm lịch {}/{} năm Bính Tý",
                chart.lunar_day, chart.lunar_month
            )
        );
    }
}
