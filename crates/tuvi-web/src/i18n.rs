//! Languages: English first, Vietnamese second.
//!
//! The interface text is in Fluent files, `locales/en.ftl` and
//! `locales/vi.ftl`, read by Topcoat's `t!` (see `main.rs`). Run
//! `topcoat l10n check` to find missing or unused messages. Data files that
//! hold both languages side by side use [`Text`].
//!
//! The root layer picks the locale of each request: the `lang` cookie, set
//! by the language switch (`POST /lang`), else the browser's languages,
//! else English.

use serde::Deserialize;
use topcoat::{
    context::Cx,
    cookie::{Cookies, cookies},
    l10n::{Locale, accept_language, locale, negotiate},
};

/// The supported locales. The first is the default.
pub static LOCALES: [Locale; 2] = [topcoat::l10n::locale!("en"), topcoat::l10n::locale!("vi")];

/// The cookie that keeps the picked language.
pub const COOKIE: &str = "lang";

/// The language of a request, for text that does not come from Fluent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    En,
    Vi,
}

impl Lang {
    /// The language of the locale in the context.
    pub fn of(cx: &Cx) -> Self {
        if *locale(cx) == LOCALES[1] {
            Self::Vi
        } else {
            Self::En
        }
    }

    /// The code in the `lang` attribute and the cookie.
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Vi => "vi",
        }
    }

    /// The other language, for the switch.
    pub fn other(self) -> Self {
        match self {
            Self::En => Self::Vi,
            Self::Vi => Self::En,
        }
    }

    /// `en` or `vi`, by language.
    pub fn pick<T>(self, en: T, vi: T) -> T {
        match self {
            Self::En => en,
            Self::Vi => vi,
        }
    }
}

/// A text in both languages, as data files store it.
#[derive(Debug, Clone, Deserialize)]
pub struct Text {
    pub en: String,
    pub vi: String,
}

impl Text {
    pub fn get(&self, lang: Lang) -> &str {
        lang.pick(&self.en, &self.vi)
    }
}

/// The locale for a request: the cookie, else the browser, else English.
pub fn request_locale(cx: &Cx) -> Locale {
    let from_cookie = cookies(cx).get(COOKIE).and_then(|cookie| {
        LOCALES
            .iter()
            .find(|locale| locale.to_string() == cookie.value())
    });
    from_cookie
        .or_else(|| negotiate(accept_language(cx), &LOCALES))
        .unwrap_or(&LOCALES[0])
        .clone()
}
