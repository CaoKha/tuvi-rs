//! The draw screen on the home page, and the redraw button of the deck.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    router::href,
    view::{View, view},
};

use tuvi_core::SolarYear;

use super::{BirthDate, loader::shuffle_loader, wheel::wheel};
use crate::{app::card::draw, features::deck::sky, t};

/// The wheels, the date check and the shuffle loader before the POST.
const GACHA_JS: Asset = asset!("./gacha.js");

/// The year wheel. The engine supports 1800–2199; the wheel keeps the years
/// people use, so it stays short.
const YEARS: std::ops::RangeInclusive<i32> = 1900..=2099;

/// The date on the wheels when the page opens.
const DEFAULT_DATE: BirthDate = BirthDate {
    day: 1,
    month: 1,
    year: SolarYear(2000),
};

/// Two digits, as the day and month wheels show them.
fn pad2(value: u8) -> String {
    format!("{value:02}")
}

/// The home page: the title, a face-down card, the date wheels and the draw
/// button. `date` and `error` come back after a date that is not on the
/// calendar.
#[topcoat::view::component]
pub async fn gacha_screen(
    cx: &Cx,
    date: Option<BirthDate>,
    error: Option<String>,
) -> Result<impl View> {
    let date = date.unwrap_or(DEFAULT_DATE);
    Ok(view! {
        <form
            method="post"
            action=(href!(draw).resolve(cx))
            autocomplete="off"
            class="grid justify-items-center gap-y-5 md:grid-cols-2 md:items-center md:gap-x-10"
            data-gacha=""
        >
            <div
                class="text-center md:col-start-1 md:row-start-1 md:self-end md:justify-self-stretch md:text-left"
            >
                <p
                    class="text-[10px] font-bold tracking-[0.28em] text-accent uppercase [text-shadow:0_0_14px_var(--accent-glow)]"
                >
                    (t!(draw_kicker))
                </p>
                <h1
                    class="mt-[9px] text-[clamp(26px,7.4vw,31px)] leading-[1.08] font-semibold tracking-[-0.01em] md:text-[clamp(30px,4.4vw,38px)]"
                >
                    (t!(draw_title_1))
                    <br>
                    (t!(draw_title_2))
                </h1>
                <p
                    class="mt-[5px] text-[13.5px] font-light text-muted md:mt-[9px] md:max-w-[36ch] md:text-[15px]"
                >
                    (t!(draw_intro))
                </p>
            </div>
            card_back()
            <div
                class="glass w-full max-w-sm px-3.5 py-3 md:col-start-1 md:row-start-2 md:max-w-none"
            >
                <div class="relative grid grid-cols-[1fr_1fr_1.15fr] gap-2.5">
                    // The selection band behind the middle row of the wheels.
                    <div
                        class="pointer-events-none absolute inset-x-1.5 bottom-[55px] h-[46px] rounded-[14px] border border-accent-glow bg-linear-115 from-accent/16 to-accent-2/16"
                        aria-hidden="true"
                    ></div>
                    wheel(
                        name: "day",
                        label: t!(draw_day).resolve(cx).into_owned(),
                        options: (1..=31).map(pad2).collect(),
                        checked: pad2(date.day)
                    )
                    wheel(
                        name: "month",
                        label: t!(draw_month).resolve(cx).into_owned(),
                        options: (1..=12).map(pad2).collect(),
                        checked: pad2(date.month)
                    )
                    wheel(
                        name: "year",
                        label: t!(draw_year).resolve(cx).into_owned(),
                        options: YEARS.map(|year| year.to_string()).collect(),
                        checked: date.year.to_string()
                    )
                </div>
            </div>
            <div class="flex w-full max-w-sm flex-col gap-2.5 md:col-start-1 md:row-start-3 md:max-w-none">
                if let Some(error) = &error {
                    <p role="alert" class="text-center text-sm text-fire">(error.as_str())</p>
                }
                <button
                    type="submit"
                    class="pill-primary h-[52px] w-full cursor-pointer rounded-full text-[15px] font-semibold"
                >
                    "✦ "
                    (t!(draw_button))
                </button>
            </div>
        </form>
        gacha_assets()
        <div
            class="pointer-events-none fixed bottom-[22px] left-1/2 z-40 w-max max-w-[calc(100%-32px)] -translate-x-1/2 translate-y-5 rounded-full border border-accent-glow bg-bg-1/95 px-[18px] py-3 text-[13.5px] opacity-0 transition data-show:translate-y-0 data-show:opacity-100"
            role="status"
            aria-live="polite"
            data-toast=""
            data-invalid=(t!(draw_toast_invalid).resolve(cx))
        ></div>
    })
}

/// A card face down: the twilight back with a quiet sky. It floats.
#[topcoat::view::component]
async fn card_back(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div
            aria-hidden="true"
            class="card-back relative flex aspect-[9/16] w-[min(170px,46vw)] flex-col items-center justify-between overflow-hidden rounded-[22px] border border-accent/40 bg-linear-168 from-bg-2 via-bg-1 to-bg-0 px-4 py-5 shadow-[0_0_40px_color-mix(in_srgb,var(--accent)_26%,transparent)] md:col-start-2 md:row-span-3 md:row-start-1 md:w-[min(300px,100%)] md:px-5 md:py-6"
        >
            <p
                class="font-display text-[11px] font-semibold tracking-[0.3em] text-muted md:text-[13px]"
            >
                "THIÊN HẠC"
            </p>
            <div class="quiet w-full">sky(class: "h-auto w-full")</div>
            <p
                class="text-[34px] leading-none text-accent drop-shadow-[0_0_18px_var(--accent-glow)] md:text-[44px]"
            >
                "✦"
            </p>
            <div class="quiet w-full rotate-180">sky(class: "h-auto w-full")</div>
            <p
                class="font-display text-[11px] font-semibold tracking-[0.3em] text-muted md:text-[13px]"
            >
                "TỬ VI"
            </p>
        </div>
    })
}

/// The redraw button of the deck: a new card on the same `date`.
#[topcoat::view::component]
pub async fn redraw_button(cx: &Cx, date: BirthDate) -> Result<impl View> {
    Ok(view! {
        <form method="post" action=(href!(draw).resolve(cx)) data-gacha="">
            <input type="hidden" name="day" value=(date.day)>
            <input type="hidden" name="month" value=(date.month)>
            <input type="hidden" name="year" value=(date.year.0)>
            <button
                type="submit"
                class="pill-primary h-[52px] w-full cursor-pointer rounded-full text-[15px] font-semibold"
            >
                "✦ "
                (t!(draw_again))
            </button>
        </form>
        gacha_assets()
    })
}

/// The shuffle loader and its script. One per page.
#[topcoat::view::component]
async fn gacha_assets(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        shuffle_loader()
        <script type="module" src=(GACHA_JS)></script>
    })
}
