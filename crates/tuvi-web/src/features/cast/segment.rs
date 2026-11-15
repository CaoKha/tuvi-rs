//! Two radio chips: the gender.

use topcoat::{
    Result,
    context::Cx,
    view::{View, view},
};

/// One pair of radio chips. `options` are `(value, label)` pairs.
#[topcoat::view::component]
pub(super) async fn segment(
    cx: &Cx,
    name: &'static str,
    label: String,
    options: [(&'static str, String); 2],
    checked: Option<&'static str>,
) -> Result<impl View> {
    Ok(view! {
        <div class="chips" role="radiogroup" aria-label=(label.as_str())>
            for (index, (value, text)) in options.into_iter().enumerate() {
                <label class="chip">
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
