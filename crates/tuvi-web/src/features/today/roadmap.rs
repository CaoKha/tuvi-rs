//! The roadmap of one day, today or the day picked in the calendar
//! (`?day=YYYY-MM-DD`): the theme of the day, its Tứ Hóa, and the 12 canh
//! giờ with what to do in each.
//!
//! Each hour gets a score: hoàng đạo +1, Hóa Lộc +2, Hóa Quyền and Hóa Khoa
//! +1, Hóa Kỵ −2. Up to three hoàng đạo hours with a score of 2 or more are
//! marked best, and every hour with Hóa Kỵ asks for care.

use serde::Deserialize;
use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::{error::not_found, parse_query_params, request::uri},
    view::{View, class, view},
};
use tuvi_core::{Chart, DayFlow, day_flow};

use super::{
    clock::{Day, Now, iso, month_page, parse_iso},
    guide::{Guide, Palace},
};
use crate::{i18n::Lang, t};

/// How many hours the roadmap marks as best.
const BEST_HOURS: usize = 3;

/// Where an hour stands against the clock.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum When {
    Past,
    Now,
    Later,
}

/// One Hóa of the day, ready to show.
struct HoaLine<'a> {
    /// The CSS color of the Hóa.
    color: &'static str,
    name: &'a str,
    /// What the Hóa means, as a tooltip.
    meaning: &'a str,
    sao: &'static str,
    palace: &'a str,
    text: &'a str,
}

/// One canh giờ, ready to show.
struct Row<'a> {
    time: String,
    can: &'static str,
    chi: &'static str,
    god: &'a str,
    auspicious: bool,
    palace: &'a str,
    /// The short names of the day's Hóa in this hour's cung.
    hoa: Vec<(&'static str, &'a str)>,
    /// The main advice line.
    text: &'a str,
    /// What the hour god favors or warns against.
    god_text: &'a str,
    theme: &'a str,
    when: When,
    best: bool,
    careful: bool,
}

/// The CSS color and the score of a Hóa.
fn hoa_style(hoa: &str) -> (&'static str, i32) {
    match hoa {
        "Hóa Lộc" => ("var(--element-moc)", 2),
        "Hóa Quyền" => ("var(--element-hoa)", 1),
        "Hóa Khoa" => ("var(--element-thuy)", 1),
        _ => ("var(--color-coral)", -2),
    }
}

/// The rows of the day. `now` is the current canh giờ, or `None` for a day
/// that is not today.
fn rows<'a>(flow: &DayFlow, guide: &'a Guide, lang: Lang, now: Option<usize>) -> Vec<Row<'a>> {
    let mut rows: Vec<_> = flow
        .gio
        .iter()
        .enumerate()
        .filter_map(|(index, gio)| {
            let palace = guide.palace(gio.role)?;
            let hour = guide.hour(gio.god)?;
            let hoa: Vec<_> = gio
                .tu_hoa
                .iter()
                .filter_map(|name| Some((*name, guide.transformation(name)?)))
                .collect();
            // Kỵ speaks first, then the other Hóa, then the hour's own mood.
            let hoa_text = hoa
                .iter()
                .find(|(name, _)| *name == "Hóa Kỵ")
                .or_else(|| hoa.first())
                .and_then(|(_, transformation)| transformation.in_palace(gio.role));
            let text = match hoa_text {
                Some(text) => text.get(lang),
                None if gio.hoang_dao => palace.good.get(lang),
                None => palace.avoid.get(lang),
            };
            let end = (gio.start_hour + 2) % 24;
            Some(Row {
                time: format!("{:02}:00–{end:02}:00", gio.start_hour),
                can: gio.can,
                chi: gio.chi,
                god: hour.name.get(lang),
                auspicious: gio.hoang_dao,
                palace: palace.name.get(lang),
                hoa: hoa
                    .iter()
                    .map(|(name, transformation)| {
                        (hoa_style(name).0, transformation.name.get(lang))
                    })
                    .collect(),
                text,
                god_text: if gio.hoang_dao {
                    hour.good.get(lang)
                } else {
                    hour.avoid.get(lang)
                },
                theme: palace.theme.get(lang),
                when: match now {
                    Some(now) if index < now => When::Past,
                    Some(now) if index == now => When::Now,
                    _ => When::Later,
                },
                best: false,
                careful: gio.tu_hoa.contains(&"Hóa Kỵ"),
            })
        })
        .collect();

    let score = |index: usize| {
        let gio = &flow.gio[index];
        i32::from(gio.hoang_dao) + gio.tu_hoa.iter().map(|hoa| hoa_style(hoa).1).sum::<i32>()
    };
    let mut ranked: Vec<_> = (0..rows.len())
        .filter(|&index| flow.gio[index].hoang_dao && score(index) >= 2)
        .collect();
    ranked.sort_by_key(|&index| -score(index));
    for index in ranked.into_iter().take(BEST_HOURS) {
        rows[index].best = true;
    }
    rows
}

/// Steps the calendar between months.
const CALENDAR_JS: Asset = asset!("./calendar.js");

/// The query of the card page: the day of the roadmap.
#[derive(Debug, Deserialize)]
struct DayQuery {
    day: Option<String>,
}

/// One day, ready to show.
struct DayView<'a> {
    /// The day count from 1970-01-01.
    n: i64,
    /// `mon` to `sun`.
    weekday: &'static str,
    date: String,
    lunar: String,
    can: &'static str,
    chi: &'static str,
    menh: &'a Palace,
    month: &'a Palace,
    hoa_lines: Vec<HoaLine<'a>>,
    rows: Vec<Row<'a>>,
}

/// The view of one day over `chart`. `now` is the current canh giờ when the
/// day is today.
fn day_view<'a>(
    chart: &Chart,
    guide: &'a Guide,
    lang: Lang,
    day: Day,
    now: Option<usize>,
) -> Option<DayView<'a>> {
    let flow = day_flow(chart, day.year, day.month, day.day).ok()?;
    let hoa_lines = flow
        .tu_hoa
        .iter()
        .filter_map(|hoa| {
            let transformation = guide.transformation(hoa.hoa)?;
            Some(HoaLine {
                color: hoa_style(hoa.hoa).0,
                name: transformation.name.get(lang),
                meaning: transformation.meaning.get(lang),
                sao: hoa.sao,
                palace: guide.palace(hoa.role)?.name.get(lang),
                text: transformation.in_palace(hoa.role)?.get(lang),
            })
        })
        .collect();
    let rows = rows(&flow, guide, lang, now);
    Some(DayView {
        n: day.n,
        weekday: day.weekday,
        date: format!("{:02}/{:02}/{}", flow.day, flow.month, flow.year),
        lunar: format!("{}/{}", flow.lunar_day, flow.lunar_month),
        can: flow.ngay_can,
        chi: flow.ngay_chi,
        menh: guide.palace(flow.menh_role)?,
        month: guide.palace(flow.month_role)?,
        hoa_lines,
        rows,
    })
}

/// The roadmap of one day over `chart`: the day in `?day=`, else today. A
/// day the engine cannot show falls back to today. The day panel has a
/// calendar button at its top right to pick another day.
#[topcoat::view::component]
pub async fn day_roadmap(cx: &Cx, chart: &Chart) -> Result<impl View> {
    let lang = Lang::of(cx);
    let guide = Guide::of(cx);
    let now = Now::vietnam();
    let today = now.tu_vi_days();
    let picked = parse_query_params::<DayQuery>(cx)
        .ok()
        .and_then(|query| parse_iso(query.day.as_deref()?));
    let build = |n: i64| {
        day_view(
            chart,
            guide,
            lang,
            Day::at(n),
            (n == today).then(|| now.canh()),
        )
    };
    let day = picked
        .and_then(build)
        .or_else(|| build(today))
        .ok_or_else(not_found)?;
    let path = uri(cx).path().to_owned();

    Ok(view! {
        day_panel(day: &day, today: today, path: path.as_str())
        <details class="mt-3 px-1 text-[12px] text-muted">
            <summary class="cursor-pointer">(t!(today_method_title))</summary>
            <p class="mt-2">(t!(today_method))</p>
        </details>
    })
}

/// The name of a day `offset` days from today: "Yesterday", "Today",
/// "Tomorrow", else the weekday.
#[topcoat::view::component]
async fn day_name(cx: &Cx, offset: i64, weekday: &'static str) -> Result<impl View> {
    Ok(view! {
        if offset == -1 {
            (t!(week_yesterday))
        } else if offset == 0 {
            (t!(today_label))
        } else if offset == 1 {
            (t!(week_tomorrow))
        } else {
            (t!(weekday_long, day: weekday))
        }
    })
}

/// The weekdays from Monday: the keys of the `weekday` messages.
const MONDAY_FIRST: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

/// One day of a calendar page.
struct CalendarDay {
    n: i64,
    day: i64,
    href: String,
}

/// The calendar button and its popover: a month page where each day links
/// to `?day=`. It opens on the month of the day shown, filled; today is
/// ringed. `calendar.js` steps between months; without it the page still
/// picks a day in this month.
#[topcoat::view::component]
async fn day_calendar(cx: &Cx, shown: i64, today: i64, path: &str) -> Result<impl View> {
    let start = Day::at(shown);
    let (month_first, length, offset) = month_page(start.year.0, start.month);
    let days: Vec<_> = (0..length)
        .map(|day| CalendarDay {
            n: month_first + day,
            day: day + 1,
            href: format!("{path}?day={}", iso(month_first + day)),
        })
        .collect();
    let year = start.year.0.to_string();

    Ok(view! {
        <button
            type="button"
            popovertarget="day-calendar"
            class="cal-button pill-quiet"
            style="anchor-name: --cal-button"
            aria-label=(t!(calendar_open))
            title=(t!(calendar_open))
        >
            <span class="cal-icon">icon(data: iconify_icon!("lucide:calendar-days"))</span>
        </button>
        <div
            id="day-calendar"
            popover="auto"
            class="cal-menu glass"
            data-calendar="month"
            data-path=(path)
            data-year=(year.as_str())
            data-month=(start.month.to_string())
            data-today=(iso(today))
            data-shown=(iso(shown))
        >
            <div class="flex items-center justify-between">
                <button
                    type="button"
                    class="cal-step"
                    data-cal-step="-1"
                    aria-label=(t!(calendar_prev))
                >
                    icon(data: iconify_icon!("lucide:chevron-left"))
                </button>
                <p class="cal-title" data-cal-title="month" aria-live="polite">
                    (t!(calendar_month, month: start.month, year: year.as_str()))
                </p>
                <button
                    type="button"
                    class="cal-step"
                    data-cal-step="1"
                    aria-label=(t!(calendar_next))
                >
                    icon(data: iconify_icon!("lucide:chevron-right"))
                </button>
            </div>
            <div class="cal-week" aria-hidden="true">
                for key in MONDAY_FIRST {
                    <span>(t!(weekday, day: key))</span>
                }
            </div>
            <div class="cal-grid" data-cal-grid="days">
                for _ in 0..offset {
                    <span></span>
                }
                for day in &days {
                    <a
                        href=(day.href.as_str())
                        aria-current=((day.n == shown).then_some("date"))
                        class=(class!(
                            "cal-day",
                            "is-today" if day.n == today,
                            "is-shown" if day.n == shown,
                        ))
                    >
                        <span>(day.day.to_string())</span>
                    </a>
                }
            </div>
            if shown != today {
                <a href=(path) class="cal-back">(t!(calendar_back))</a>
            }
        </div>
        <script type="module" src=(CALENDAR_JS)></script>
    })
}

/// One day in two panels: its date (with the calendar at the top right),
/// palace and Tứ Hóa; then its 12 canh giờ.
#[topcoat::view::component]
async fn day_panel<'a>(cx: &Cx, day: &DayView<'a>, today: i64, path: &str) -> Result<impl View> {
    let lang = Lang::of(cx);

    Ok(view! {
        <section class="glass p-5 lg:p-6" aria-labelledby="day-title">
            <div class="flex items-start justify-between gap-3">
                <div class="min-w-0">
                    <p class="text-[10px] font-bold tracking-[0.26em] text-accent uppercase">
                        day_name(offset: day.n - today, weekday: day.weekday)
                    </p>
                    <h2 id="day-title" class="mt-1 text-2xl font-semibold tabular-nums">
                        (day.date.as_str())
                    </h2>
                    <p class="mt-1 text-[13px] text-muted">
                        (t!(today_lunar, date: day.lunar.as_str(), can: day.can, chi: day.chi))
                    </p>
                </div>
                day_calendar(shown: day.n, today: today, path: path)
            </div>

            <div class="mt-4 rounded-[18px] border border-accent/30 bg-accent/8 p-4">
                <p class="text-[11px] font-semibold tracking-[0.18em] text-muted uppercase">
                    (t!(day_palace_label))
                </p>
                <p class="mt-1 font-display text-xl font-semibold text-accent">
                    (day.menh.name.get(lang))
                </p>
                <p class="mt-1 text-[13.5px] leading-relaxed">(day.menh.theme.get(lang))</p>
                <p class="mt-1 text-[12px] text-muted">
                    (t!(today_month_palace, palace: day.month.name.get(lang)))
                </p>
                <dl class="mt-3 grid gap-2 text-[13px] leading-relaxed sm:grid-cols-2">
                    <div>
                        <dt class="font-semibold text-accent">"✓ " (t!(today_do))</dt>
                        <dd class="text-muted">(day.menh.good.get(lang))</dd>
                    </div>
                    <div>
                        <dt class="font-semibold text-coral">"✕ " (t!(today_avoid))</dt>
                        <dd class="text-muted">(day.menh.avoid.get(lang))</dd>
                    </div>
                </dl>
            </div>

            <h3 class="mt-5 font-sans text-[11px] font-semibold tracking-[0.18em] text-muted uppercase">
                (t!(today_hoa_title, can: day.can))
            </h3>
            <ul class="mt-2 grid gap-2.5">
                for line in &day.hoa_lines {
                    <li class="flex gap-3 text-[13px] leading-relaxed">
                        <span
                            class="mt-1.5 size-2 shrink-0 rounded-full bg-(--hoa) shadow-[0_0_8px_var(--hoa)]"
                            style=(format!("--hoa: {}", line.color))
                        ></span>
                        <span>
                            <b class="font-semibold text-ink" title=(line.meaning)>(line.name)</b>
                            <span class="text-muted">" · " (line.sao) " · " (line.palace)</span>
                            <br>
                            <span class="text-muted">(line.text)</span>
                        </span>
                    </li>
                }
            </ul>
        </section>

        <section class="glass mt-4 p-5 lg:p-6" aria-labelledby="hours-title">
            <h2 id="hours-title" class="text-lg font-semibold">(t!(today_timeline_title))</h2>
            <p class="mt-1 text-[12.5px] text-muted">(t!(today_timeline_hint))</p>
            <ol class="relative mt-4 grid gap-2 before:absolute before:top-2 before:bottom-2 before:left-[7px] before:w-px before:bg-glass-line before:content-['']">
                for row in &day.rows {
                    hour_row(row: row)
                }
            </ol>
        </section>
    })
}

/// One canh giờ. The summary shows the advice; the body shows the hour god
/// and the theme of the cung.
#[topcoat::view::component]
async fn hour_row<'a>(cx: &Cx, row: &Row<'a>) -> Result<impl View> {
    Ok(view! {
        <li
            class=(class!(
                "relative pl-6",
                "opacity-55" if row.when == When::Past,
            ))
        >
            <span
                aria-hidden="true"
                class=(class!(
                    "absolute top-3.5 left-0 size-[15px] rounded-full border-2",
                    "border-accent bg-accent shadow-[0_0_12px_var(--accent-glow)]" if row.when == When::Now,
                    "border-accent bg-bg-0" if row.when != When::Now && row.auspicious,
                    "border-glass-line bg-bg-0" if row.when != When::Now && !row.auspicious,
                ))
            ></span>
            <details
                class=(class!(
                    "group rounded-[16px] border px-3.5 py-3",
                    "border-accent bg-accent/10" if row.when == When::Now,
                    "border-glass-line bg-white/[0.03]" if row.when != When::Now,
                ))
                open=(row.when == When::Now)
            >
                <summary class="cursor-pointer list-none">
                    <div class="flex flex-wrap items-center gap-x-2 gap-y-1">
                        <span class="font-display text-[15px] font-semibold tabular-nums">
                            (row.time.as_str())
                        </span>
                        <span class="text-[12px] text-muted">
                            (t!(today_hour, can: row.can, chi: row.chi))
                        </span>
                        if row.when == When::Now {
                            <span class="pill-primary rounded-full px-2 py-0.5 text-[10px] font-bold uppercase">
                                (t!(today_now))
                            </span>
                        }
                        if row.best {
                            <span class="rounded-full border border-accent/60 px-2 py-0.5 text-[10px] font-bold text-accent uppercase">
                                "✦ " (t!(today_best))
                            </span>
                        }
                        if row.careful {
                            <span class="rounded-full border border-coral/60 px-2 py-0.5 text-[10px] font-bold text-coral uppercase">
                                (t!(today_careful))
                            </span>
                        }
                    </div>
                    <div class="mt-1 flex flex-wrap items-center gap-x-2 gap-y-1 text-[12px]">
                        <span class=(if row.auspicious { "text-accent" } else { "text-muted" })>
                            (row.god)
                            " · "
                            if row.auspicious {
                                (t!(today_auspicious))
                            } else {
                                (t!(today_inauspicious))
                            }
                        </span>
                        <span class="text-muted">"·"</span>
                        <span class="font-semibold text-ink">(row.palace)</span>
                        for (color, name) in &row.hoa {
                            <span
                                class="rounded-full border border-(--hoa)/60 px-1.5 text-[10.5px] font-semibold text-(--hoa)"
                                style=(format!("--hoa: {color}"))
                            >
                                (*name)
                            </span>
                        }
                    </div>
                    <p class="mt-1.5 text-[13px] leading-relaxed">(row.text)</p>
                </summary>
                <div class="mt-2 border-t border-glass-line pt-2 text-[12.5px] leading-relaxed text-muted">
                    <p>
                        <b class="font-semibold text-ink">(row.god)</b>
                        ": "
                        (row.god_text)
                    </p>
                    <p class="mt-1">
                        <b class="font-semibold text-ink">(row.palace)</b>
                        ": "
                        (row.theme)
                    </p>
                </div>
            </details>
        </li>
    })
}

#[cfg(test)]
mod tests {
    use tuvi_core::{BirthInput, Gender, SolarYear, build_chart};

    use super::*;

    fn flow() -> DayFlow {
        let chart = build_chart(BirthInput {
            year: SolarYear(1996),
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nu,
        })
        .unwrap();
        day_flow(&chart, SolarYear(2026), 11, 15).unwrap()
    }

    #[test]
    fn every_hour_has_text_in_both_languages() {
        let guide = Guide::load().unwrap();
        let flow = flow();
        for lang in [Lang::En, Lang::Vi] {
            let rows = rows(&flow, &guide, lang, Some(4));
            assert_eq!(rows.len(), 12);
            for row in &rows {
                assert!(!row.text.is_empty() && !row.god_text.is_empty() && !row.palace.is_empty());
            }
            assert_eq!(rows[3].when, When::Past);
            assert_eq!(rows[4].when, When::Now);
            assert_eq!(rows[5].when, When::Later);
        }
    }

    #[test]
    fn best_hours_and_cautions_follow_the_scores() {
        let guide = Guide::load().unwrap();
        let flow = flow();
        let rows = rows(&flow, &guide, Lang::En, None);
        let best = rows.iter().filter(|row| row.best).count();
        assert!(best <= BEST_HOURS);
        for (row, gio) in rows.iter().zip(&flow.gio) {
            assert_eq!(row.careful, gio.tu_hoa.contains(&"Hóa Kỵ"));
            if row.best {
                assert!(gio.hoang_dao);
            }
        }
        assert_eq!(rows[0].time, "23:00–01:00");
        assert_eq!(rows[11].time, "21:00–23:00");
    }
}
