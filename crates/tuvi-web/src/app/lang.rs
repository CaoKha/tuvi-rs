//! `POST /lang`: the language switch. It keeps the pick in a cookie for a
//! year, then goes back to the page.

use serde::Deserialize;
use topcoat::{
    Result,
    context::Cx,
    cookie::{Cookie, Cookies, SameSite, cookies, time::Duration},
    router::{
        content::Form,
        error::{SeeOther, see_other},
        route,
    },
};

use crate::i18n::{COOKIE, LOCALES};

#[derive(Deserialize)]
pub struct Switch {
    lang: String,
    /// The path of the page to go back to.
    back: String,
}

#[route(POST)]
pub async fn switch(cx: &Cx, Form(form): Form<Switch>) -> Result<SeeOther> {
    if LOCALES.iter().any(|locale| locale.to_string() == form.lang) {
        cookies(cx).add(
            Cookie::build((COOKIE, form.lang))
                .path("/")
                .max_age(Duration::days(365))
                .same_site(SameSite::Lax),
        );
    }
    Ok(see_other(safe_back(&form.back)))
}

/// `back` when it is a path on this site that a GET can open, else `/`. A
/// path like `//evil.example` would leave the site.
pub(super) fn safe_back(back: &str) -> &str {
    let local = back.starts_with('/') && !back.starts_with("//") && !back.contains('\\');
    // `/card` only takes the draw POST.
    if local && back != "/card" { back } else { "/" }
}

#[cfg(test)]
mod tests {
    use super::safe_back;

    #[test]
    fn only_local_paths_come_back() {
        assert_eq!(
            safe_back("/card/1996-04-15-07-nu"),
            "/card/1996-04-15-07-nu"
        );
        assert_eq!(safe_back("//evil.example"), "/");
        assert_eq!(safe_back("https://evil.example"), "/");
        assert_eq!(safe_back("/\\evil.example"), "/");
        assert_eq!(safe_back("/card"), "/");
    }
}
