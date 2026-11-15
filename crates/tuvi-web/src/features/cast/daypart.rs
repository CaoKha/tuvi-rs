//! AM and PM: two chips, a sun and a moon.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::Signal,
    view::{View, view},
};

/// One half of the day: a chip with its sun or moon. `value` is "AM" or
/// "PM". `buoi` holds the checked value.
#[topcoat::view::component]
pub(super) async fn daypart(
    cx: &Cx,
    value: &'static str,
    checked: bool,
    buoi: &Signal<String>,
) -> Result<impl View> {
    let pm = value == "PM";
    Ok(view! {
        <label class="chip">
            <input
                type="radio"
                name="buoi"
                value=(value)
                required=(!pm)
                checked=(checked)
                class="sr-only"
                @change=$(|_e| buoi.set(if pm { "PM".to_owned() } else { "AM".to_owned() }))
            >
            <span class="inline-flex" aria-hidden="true">
                if pm {
                    icon(data: iconify_icon!("lucide:moon"))
                } else {
                    icon(data: iconify_icon!("lucide:sun"))
                }
            </span>
            (value)
        </label>
    })
}
