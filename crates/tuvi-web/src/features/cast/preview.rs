//! The live preview on the card: the chính tinh of cung Mệnh for the picked
//! date and hour. Gender does not move cung Mệnh, so the preview does not
//! wait for it.
//!
//! The browser re-renders the shard each time a wheel settles or the clock
//! moves. The arguments come from the browser, so the shard parses and checks
//! each one, and `tuvi_core::build_chart` checks the date.

use topcoat::{
    Result,
    context::Cx,
    runtime::shard,
    view::{View, class, view},
};
use tuvi_core::{BirthInput, ChartError, Gender, SolarYear, build_chart};

use super::birth::{Buoi, hour_24};
use crate::{
    features::{
        charts::{accent_style, menh_chinh_tinh},
        deck::{chom_figure, chom_sao_of, sky},
    },
    t,
};

/// What the preview shows.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Preview {
    /// No hour yet: the card stays face down.
    Asleep,
    /// The date is not on the calendar.
    OffCalendar,
    /// The year is outside 1800–2199.
    OutOfRange,
    /// The chart builds. `chinh_tinh` is empty for vô chính diệu.
    Menh {
        chinh_tinh: Vec<&'static str>,
        accent: Option<String>,
    },
}

/// Builds the preview from the raw shard arguments. Never fails: bad input
/// gives a state to show.
pub(super) fn preview(day: &str, month: &str, year: &str, gio: usize, pm: bool) -> Preview {
    let buoi = if pm { Buoi::Chieu } else { Buoi::Sang };
    let Some(hour) = u8::try_from(gio).ok().and_then(|gio| hour_24(gio, buoi)) else {
        return Preview::Asleep;
    };
    let (Ok(day), Ok(month), Ok(year)) = (
        day.trim().parse::<u8>(),
        month.trim().parse::<u8>(),
        year.trim().parse::<i32>(),
    ) else {
        return Preview::OffCalendar;
    };
    let birth = BirthInput {
        year: SolarYear(year),
        month,
        day,
        hour,
        // Gender changes the Đại hạn direction, not cung Mệnh.
        gender: Gender::Nam,
    };
    match build_chart(birth) {
        Ok(chart) => Preview::Menh {
            chinh_tinh: menh_chinh_tinh(&chart).to_vec(),
            accent: accent_style(&chart),
        },
        Err(ChartError::InvalidDate) => Preview::OffCalendar,
        Err(ChartError::YearOutOfRange) => Preview::OutOfRange,
    }
}

/// The middle of the card: the sky or the constellations, and a caption. It
/// also sets the page accent with a `<style>` rule, so the whole screen
/// takes the color of the chart.
#[shard("/cast/preview")]
pub async fn menh_preview(
    cx: &Cx,
    day: String,
    month: String,
    year: String,
    gio: usize,
    pm: bool,
) -> Result<impl View> {
    let state = preview(&day, &month, &year, gio, pm);
    let lit = matches!(state, Preview::Menh { .. });
    let (names, accent) = match &state {
        Preview::Menh { chinh_tinh, accent } => (chinh_tinh.join(" · "), accent.clone()),
        _ => (String::new(), None),
    };
    let caption = match (&state, names.is_empty()) {
        (Preview::Asleep, _) => t!(cast_preview_asleep).resolve(cx).into_owned(),
        (Preview::OffCalendar, _) => t!(cast_preview_off_calendar).resolve(cx).into_owned(),
        (Preview::OutOfRange, _) => t!(cast_preview_out_of_range).resolve(cx).into_owned(),
        (Preview::Menh { .. }, true) => t!(cast_preview_empty_menh).resolve(cx).into_owned(),
        (Preview::Menh { .. }, false) => t!(cast_preview_awake, names: names.as_str())
            .resolve(cx)
            .into_owned(),
    };
    // The figures of the Mệnh stars. Vô chính diệu keeps the sky.
    let figures: Vec<_> = match &state {
        Preview::Menh { chinh_tinh, .. } => chinh_tinh
            .iter()
            .filter_map(|name| chom_sao_of(name))
            .collect(),
        _ => Vec::new(),
    };
    let song_tinh = figures.len() > 1;

    Ok(view! {
        <div
            class=(class!("flex w-full flex-1 flex-col items-center justify-center gap-2", "lit" if lit, "quiet" if names.is_empty()))
            role="status"
            aria-live="polite"
        >
            if let Some(accent) = &accent {
                <style>
                    ":root { "
                    (accent.as_str())
                    " }"
                </style>
            }
            if figures.is_empty() {
                sky(class: "h-auto w-full")
                if !lit {
                    <p
                        class="text-[40px] leading-none text-accent drop-shadow-[0_0_18px_var(--accent-glow)]"
                        aria-hidden="true"
                    >
                        "✦"
                    </p>
                }
                sky(class: "h-auto w-full rotate-180")
            } else {
                <div class="flex w-full justify-center gap-0.5">
                    for (index, chom) in figures.into_iter().enumerate() {
                        chom_figure(chom: chom, song_tinh: song_tinh, delay_ms: index * 90)
                    }
                </div>
            }
            <p
                class=(class!(
                    "px-1 text-center text-muted uppercase",
                    "text-[9.5px] leading-[1.45] tracking-[0.08em]" if song_tinh else "text-[10.5px] tracking-[0.16em]",
                ))
            >
                (caption)
            </p>
        </div>
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_hour_sleeps() {
        assert_eq!(preview("1", "1", "2000", 0, false), Preview::Asleep);
        assert_eq!(preview("1", "1", "2000", 13, true), Preview::Asleep);
    }

    #[test]
    fn bad_dates_have_their_own_state() {
        assert_eq!(preview("31", "2", "2000", 3, false), Preview::OffCalendar);
        assert_eq!(preview("x", "2", "2000", 3, false), Preview::OffCalendar);
        assert_eq!(preview("1", "1", "2300", 3, false), Preview::OutOfRange);
    }

    #[test]
    fn a_real_date_shows_the_menh_stars() {
        let birth = BirthInput {
            year: SolarYear(1996),
            month: 4,
            day: 15,
            hour: 22,
            gender: Gender::Nam,
        };
        let chart = build_chart(birth).unwrap();
        assert_eq!(
            preview("15", "04", "1996", 10, true),
            Preview::Menh {
                chinh_tinh: menh_chinh_tinh(&chart).to_vec(),
                accent: accent_style(&chart),
            }
        );
    }
}
