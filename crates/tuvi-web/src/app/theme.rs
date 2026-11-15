//! `POST /theme`: the night and day switch. It keeps the pick in a cookie
//! for a year, then goes back to the page. With JavaScript the switch
//! happens in place instead (`features/shell/theme.js`).

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

use super::lang::safe_back;
use crate::features::shell::THEME_COOKIE;

#[derive(Deserialize)]
pub struct Switch {
    theme: String,
    /// The path of the page to go back to.
    back: String,
}

#[route(POST)]
pub async fn switch(cx: &Cx, Form(form): Form<Switch>) -> Result<SeeOther> {
    if form.theme == "dark" || form.theme == "light" {
        cookies(cx).add(
            Cookie::build((THEME_COOKIE, form.theme))
                .path("/")
                .max_age(Duration::days(365))
                .same_site(SameSite::Lax),
        );
    }
    Ok(see_other(safe_back(&form.back)))
}
