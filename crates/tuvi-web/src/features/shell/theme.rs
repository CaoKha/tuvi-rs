//! The theme: night (the default, the twilight design) or day (a calm dawn
//! sky). A cookie keeps the pick, so the server renders the page in it and
//! nothing flashes. The switch is a round button next to the language menu:
//! a form that posts to `/theme`, which `theme.js` turns into an instant
//! switch without a reload.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    cookie::{Cookies, cookies},
    icon::{icon, iconify::iconify_icon},
    router::{href, request::uri},
    view::{View, view},
};

use crate::t;

/// The cookie that keeps the theme: `dark` or `light`.
pub const COOKIE: &str = "theme";

/// Switches the theme in place.
const THEME_JS: Asset = asset!("./theme.js");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Theme {
    Night,
    Day,
}

impl Theme {
    /// The theme of a request: the cookie, else night.
    pub fn of(cx: &Cx) -> Self {
        match cookies(cx).get(COOKIE) {
            Some(cookie) if cookie.value() == "light" => Self::Day,
            _ => Self::Night,
        }
    }

    /// The class on `<html>` and the cookie value.
    pub fn code(self) -> &'static str {
        match self {
            Self::Night => "dark",
            Self::Day => "light",
        }
    }

    /// The browser bar color.
    pub fn bar_color(self) -> &'static str {
        match self {
            Self::Night => "#1a1033",
            Self::Day => "#d8cbea",
        }
    }

    fn other(self) -> Self {
        match self {
            Self::Night => Self::Day,
            Self::Day => Self::Night,
        }
    }
}

/// The switch: a sun at night (to the day), a moon by day (to the night).
#[topcoat::view::component]
pub async fn theme_toggle(cx: &Cx, theme: Theme) -> Result<impl View> {
    let back = uri(cx)
        .path_and_query()
        .map_or("/", |path| path.as_str())
        .to_owned();
    let to_day = t!(theme_to_day).resolve(cx).into_owned();
    let to_night = t!(theme_to_night).resolve(cx).into_owned();
    let label = match theme {
        Theme::Night => to_day.clone(),
        Theme::Day => to_night.clone(),
    };
    Ok(view! {
        <form method="post" action=(href!(crate::app::theme::switch).resolve(cx)) data-theme-form="">
            <input type="hidden" name="theme" value=(theme.other().code())>
            <input type="hidden" name="back" value=(back.as_str())>
            <button
                type="submit"
                aria-label=(label.as_str())
                title=(label.as_str())
                data-to-day=(to_day.as_str())
                data-to-night=(to_night.as_str())
                class="theme-toggle pill-quiet inline-flex h-9 w-8 cursor-pointer items-center justify-center rounded-full text-base text-accent"
            >
                <span class="theme-sun">icon(data: iconify_icon!("lucide:sun"))</span>
                <span class="theme-moon">icon(data: iconify_icon!("lucide:moon"))</span>
            </button>
        </form>
        <script type="module" src=(THEME_JS)></script>
    })
}
