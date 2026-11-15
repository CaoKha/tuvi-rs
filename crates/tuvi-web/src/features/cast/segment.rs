//! A row of round radio segments: AM/PM and gender.

use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, Signal},
    view::{View, class, view},
};

/// One segmented control. `options` are `(value, label)` pairs. `picked`,
/// when given, holds the checked value.
#[topcoat::view::component]
pub(super) async fn segment(
    cx: &Cx,
    name: &'static str,
    label: String,
    options: [(&'static str, String); 2],
    checked: Option<&'static str>,
    picked: Option<&Signal<String>>,
    #[default] class: &'static str,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!("segment", class))
            role="radiogroup"
            aria-label=(label.as_str())
            if let Some(picked) = picked {
                @change=$(|e: Event| picked.set(e.target.value))
            }
        >
            for (index, (value, text)) in options.into_iter().enumerate() {
                <label>
                    <input
                        type="radio"
                        name=(name)
                        value=(value)
                        required=(index == 0)
                        checked=(checked == Some(value))
                        class="sr-only"
                    >
                    (text.as_str())
                </label>
            }
        </div>
    })
}
