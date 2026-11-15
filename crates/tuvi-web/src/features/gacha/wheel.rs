//! A scroll-snap wheel of radios: the day, month or year.

use topcoat::{
    Result,
    context::Cx,
    view::{View, view},
};

/// One wheel. `options` are the form values, shown as they are. `checked`
/// is the value picked when the page opens.
#[topcoat::view::component]
pub(super) async fn wheel(
    cx: &Cx,
    name: &'static str,
    label: String,
    options: Vec<String>,
    checked: String,
) -> Result<impl View> {
    Ok(view! {
        <div>
            <p class="dial-label">(label.as_str())</p>
            <div class="wheel" role="radiogroup" aria-label=(label.as_str()) data-wheel="">
                <div class="h-[55px]" aria-hidden="true"></div>
                for option in options {
                    <label>
                        <input
                            type="radio"
                            name=(name)
                            value=(option.as_str())
                            checked=(option == checked)
                            class="sr-only"
                        >
                        (option.as_str())
                    </label>
                }
                <div class="h-[55px]" aria-hidden="true"></div>
            </div>
        </div>
    })
}
