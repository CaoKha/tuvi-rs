//! The home page: the title, the birth form (date wheels, birth clock with
//! AM/PM, gender), the draw button, and the card on the right. The card is
//! face down until the hour is picked; then the preview shard shows the
//! constellation of cung Mệnh on it.
//!
//! Every control is a native radio inside one `<form>`, so the form posts
//! without JavaScript. Signals copy the picked values for the clock readout
//! and the preview shard.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    router::href,
    runtime::{expr, signal},
    view::{View, view},
};

use super::{
    birth::{BirthForm, gender_value},
    clock::birth_clock,
    loader::casting_loader,
    preview::menh_preview,
    segment::segment,
    wheel::wheel,
};
use crate::{app::card::draw, t};

/// Wheel scroll, clock drag, the toasts and the loader delay.
const CAST_JS: Asset = asset!("./cast.js");

/// The year wheel. The engine supports 1800–2199; the wheel keeps the years
/// people use, so it stays short.
const YEARS: std::ops::RangeInclusive<i32> = 1900..=2099;
const YEAR_DEFAULT: i32 = 2000;

/// Two digits, as the day and month wheels show them.
fn pad2(value: u8) -> String {
    format!("{value:02}")
}

/// The home page. `sent` and `error` come back after a failed submit, or
/// `sent` fills the form from a card.
#[topcoat::view::component]
pub async fn cast_screen(
    cx: &Cx,
    sent: Option<&BirthForm>,
    error: Option<String>,
) -> Result<impl View> {
    // The date parts stay strings, as the radios give them. The preview
    // shard parses and checks them on the server.
    let day = signal(cx, || pad2(sent.map_or(1, |sent| sent.day)));
    let month = signal(cx, || pad2(sent.map_or(1, |sent| sent.month)));
    let year = signal(cx, || {
        sent.map_or(YEAR_DEFAULT, |sent| sent.year.0).to_string()
    });
    let buoi = signal(cx, || {
        sent.map_or("AM", |sent| sent.buoi.value()).to_owned()
    });
    // Clock hour 1–12, or 0 before the first pick.
    let gio = signal(cx, || sent.and_then(|sent| sent.gio).map_or(0, usize::from));
    let pm = expr!(buoi.get() == "PM");

    Ok(view! {
        <form
            method="post"
            action=(href!(draw).resolve(cx))
            autocomplete="off"
            class="cast-stage grid justify-items-center gap-y-5 lg:grid-cols-[1fr_minmax(0,24rem)_var(--stage-card)_1fr] lg:grid-rows-[auto_1fr] lg:justify-items-stretch lg:gap-x-20 lg:gap-y-0"
            data-cast=""
        >
            <div class="text-center lg:col-span-4 lg:col-start-1 lg:row-start-1 lg:mb-6">
                <p
                    class="text-[10px] font-bold tracking-[0.28em] text-accent uppercase [text-shadow:0_0_14px_var(--accent-glow)]"
                >
                    (t!(draw_kicker))
                </p>
                <h1
                    class="mt-[9px] text-[clamp(26px,7.4vw,31px)] leading-[1.08] font-semibold tracking-[-0.01em] lg:text-[42px]"
                >
                    (t!(draw_title_1))
                    " "
                    <br class="lg:hidden">
                    (t!(draw_title_2))
                </h1>
                <p
                    class="mt-[5px] text-[13.5px] font-light text-muted lg:mx-auto lg:mt-2.5 lg:max-w-[62ch] lg:text-[15px]"
                >
                    (t!(draw_intro))
                </p>
            </div>

            // From `lg` one glass table holds the birth form and the card,
            // side by side in the middle. It hugs them: the two middle
            // columns plus 2.5rem on each side.
            <div
                class="hidden lg:glass lg:col-span-2 lg:col-start-2 lg:row-start-2 lg:-mx-10 lg:block lg:shadow-[0_30px_80px_-30px_color-mix(in_srgb,var(--accent)_35%,transparent)]"
                aria-hidden="true"
            ></div>

            // The card and the main button. On a phone they are two cells
            // (`contents`): the card under the title, the button last. From
            // `lg` they are the right half of the table.
            <div
                class="contents lg:relative lg:col-start-3 lg:row-start-2 lg:flex lg:flex-col lg:justify-center lg:gap-4 lg:py-6"
            >
                <div class="flex w-full justify-center">
                    <div
                        class="card-back card-face relative flex aspect-[9/16] w-full max-w-sm flex-col items-center justify-between overflow-hidden rounded-[22px] border border-accent/40 bg-linear-168 from-bg-2 via-bg-1 to-bg-0 px-6 py-7 shadow-[0_0_40px_color-mix(in_srgb,var(--accent)_26%,transparent)] lg:w-(--stage-card) lg:max-w-none"
                    >
                        <p
                            class="font-display text-[13px] font-semibold tracking-[0.3em] text-muted"
                            aria-hidden="true"
                        >
                            "THIÊN HẠC"
                        </p>
                        menh_preview(
                            day: $(day.get()),
                            month: $(month.get()),
                            year: $(year.get()),
                            gio: $(gio.get()),
                            pm: $(pm)
                        )
                        <p
                            class="font-display text-[13px] font-semibold tracking-[0.3em] text-muted"
                            aria-hidden="true"
                        >
                            "TỬ VI"
                        </p>
                    </div>
                </div>

                <div class="order-last flex w-full max-w-sm flex-col gap-2.5 lg:order-none">
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
            </div>

            // The birth form: the date, the hour, the gender. A glass card
            // on a phone; from `lg` it is the left half of the table, so its
            // own glass goes.
            <div
                class="glass w-full max-w-sm px-3.5 py-3 lg:relative lg:col-start-2 lg:row-start-2 lg:max-w-none lg:self-center lg:border-transparent lg:bg-transparent lg:px-0 lg:py-6 lg:backdrop-blur-none"
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
                        picked: &day
                    )
                    wheel(
                        name: "month",
                        label: t!(draw_month).resolve(cx).into_owned(),
                        options: (1..=12).map(pad2).collect(),
                        picked: &month
                    )
                    wheel(
                        name: "year",
                        label: t!(draw_year).resolve(cx).into_owned(),
                        options: YEARS.map(|year| year.to_string()).collect(),
                        picked: &year
                    )
                </div>

                <p class="dial-label mt-3">
                    (t!(cast_hour_label))
                    " "
                    <span class="text-accent">(t!(cast_required))</span>
                </p>
                segment(
                    name: "buoi",
                    label: t!(cast_buoi_aria).resolve(cx).into_owned(),
                    options: [("AM", "AM".to_owned()), ("PM", "PM".to_owned())],
                    checked: Some(sent.map_or("AM", |sent| sent.buoi.value())),
                    picked: Some(&buoi),
                    class: "mx-auto w-[min(218px,66vw)]"
                )
                birth_clock(checked: sent.and_then(|sent| sent.gio), gio: &gio, buoi: &buoi)
                <p class="mt-2 text-center text-[11px] text-muted">(t!(cast_clock_hint))</p>

                <div class="mt-3.5" data-gender="">
                    <p class="dial-label">(t!(cast_gender_label))</p>
                    segment(
                        name: "gender",
                        label: t!(cast_gender_label).resolve(cx).into_owned(),
                        options: [
                            ("Nam", t!(chart_male).resolve(cx).into_owned()),
                            ("Nu", t!(chart_female).resolve(cx).into_owned()),
                        ],
                        // Male by default, so the hour is the only pick left.
                        checked: Some(sent.and_then(|sent| sent.gender).map_or("Nam", gender_value)),
                        picked: None
                    )
                </div>
            </div>
        </form>

        casting_loader()
        <div
            class="pointer-events-none fixed bottom-[22px] left-1/2 z-40 w-max max-w-[calc(100%-32px)] -translate-x-1/2 translate-y-5 rounded-full border border-accent-glow bg-bg-1/95 px-[18px] py-3 text-[13.5px] opacity-0 transition data-show:translate-y-0 data-show:opacity-100"
            role="status"
            aria-live="polite"
            data-toast=""
            data-invalid-date=(t!(draw_toast_invalid).resolve(cx))
            data-no-hour=(t!(cast_toast_no_hour).resolve(cx))
            data-no-gender=(t!(cast_toast_no_gender).resolve(cx))
        ></div>
        <script type="module" src=(CAST_JS)></script>
    })
}
