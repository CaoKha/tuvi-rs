//! The draw screen on the home page, and the draw button.

use topcoat::{
    Result,
    asset::{Asset, asset},
    context::Cx,
    router::href,
    view::{View, class, view},
};

use super::loader::shuffle_loader;
use crate::{app::la_bai::draw, features::ban_menh::sky};

/// Holds the POST until the loader has played.
const GACHA_JS: Asset = asset!("./gacha.js");

/// The home page for a logged-in user: the title, a face-down card and the
/// draw button.
#[topcoat::view::component]
pub async fn gacha_screen(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class="grid justify-items-center gap-y-6 md:grid-cols-2 md:items-center md:gap-x-10">
            <div class="text-center md:col-start-1 md:row-start-1 md:justify-self-start md:self-end md:text-left">
                <p
                    class="text-[10px] font-bold tracking-[0.28em] text-accent uppercase [text-shadow:0_0_14px_var(--accent-glow)]"
                >
                    "Gieo quẻ · rút lá bài"
                </p>
                <h1
                    class="mt-[9px] text-[clamp(26px,7.4vw,31px)] leading-[1.08] font-semibold tracking-[-0.01em] md:text-[clamp(30px,4.4vw,38px)]"
                >
                    "Rút một lá bài"
                    <br>
                    "từ bầu trời"
                </h1>
                <p
                    class="mt-[5px] text-[13.5px] font-light text-muted md:mt-[9px] md:max-w-[34ch] md:text-[15px]"
                >
                    "Mỗi lá là một ngày giờ có thật trên lịch. Thiên Hạc an sao cho ngày giờ ấy, đúng như một lá số Tử Vi."
                </p>
            </div>
            card_back()
            <div class="w-full max-w-sm md:col-start-1 md:row-start-2 md:self-start md:justify-self-start">
                draw_button(label: "✦ Rút lá bài", class: "w-full")
            </div>
        </section>
    })
}

/// A card face down: the twilight back with a quiet sky. It floats.
#[topcoat::view::component]
async fn card_back(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div
            aria-hidden="true"
            class="card-back relative flex aspect-[9/16] w-[min(240px,62vw)] flex-col items-center justify-between overflow-hidden rounded-[22px] border border-accent/40 bg-linear-168 from-bg-2 via-bg-1 to-bg-0 px-5 py-6 shadow-[0_0_40px_color-mix(in_srgb,var(--accent)_26%,transparent)] md:col-start-2 md:row-span-2 md:row-start-1 md:w-[min(300px,100%)]"
        >
            <p class="font-display text-[13px] font-semibold tracking-[0.3em] text-muted">
                "THIÊN HẠC"
            </p>
            <div class="quiet w-full">sky(class: "h-auto w-full")</div>
            <p
                class="text-[44px] leading-none text-accent drop-shadow-[0_0_18px_var(--accent-glow)]"
            >
                "✦"
            </p>
            <div class="quiet w-full rotate-180">sky(class: "h-auto w-full")</div>
            <p class="font-display text-[13px] font-semibold tracking-[0.3em] text-muted">
                "TỬ VI"
            </p>
        </div>
    })
}

/// The draw button: a POST to `/la-bai`, then the deck of the new card. It
/// brings the shuffle loader and its script, so put one per page.
#[topcoat::view::component]
pub async fn draw_button(cx: &Cx, label: &'static str, class: &'static str) -> Result<impl View> {
    Ok(view! {
        <form method="post" action=(href!(draw).resolve(cx)) data-gacha="">
            <button
                type="submit"
                class=(class!("pill-primary h-[52px] cursor-pointer rounded-full px-8 text-[15px] font-semibold", class))
            >
                (label)
            </button>
        </form>
        shuffle_loader()
        <script type="module" src=(GACHA_JS)></script>
    })
}
