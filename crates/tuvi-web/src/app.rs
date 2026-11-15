use serde_json::json;
use topcoat::{
    Result,
    asset::{Asset, AssetBundle, RouterBuilderAssetExt, asset, asset_config},
    context::Cx,
    cookie::CookieLayer,
    font::{Font, RouterBuilderFontExt, fontsource::fontsource_font},
    icon::{icon, iconify::iconify_icon},
    router::{
        Body, Next, RouterBuilder, Slot, content::Json, href, layer, layout, module_router, page,
        request::uri, response::Response, route,
    },
    runtime::RuntimeLayer,
    tailwind,
    view::{View, class, view},
};

use crate::{
    features::{
        cast::{cast_screen, menh_preview},
        shell::{LOGO, Theme, brand, theme_toggle},
    },
    i18n::{Lang, request_locale},
    t,
};

pub mod card;
pub mod lang;
pub mod theme;

/// Display font for titles and star names.
const DISPLAY_FONT: Font = fontsource_font!(
    FRAUNCES,
    weight: [400, 600],
    style: Normal,
    subset: [Latin, Vietnamese],
);

/// Body font. It covers every Vietnamese diacritic.
const BODY_FONT: Font = fontsource_font!(
    BE_VIETNAM_PRO,
    weight: [300, 400, 600, 700],
    style: Normal,
    subset: [Latin, Vietnamese],
);

/// Browser tab icons and the home screen icons, from the old app.
const FAVICON: Asset = asset!("assets/icons/favicon.ico");
const FAVICON_16: Asset = asset!("assets/icons/favicon-16x16.png");
const FAVICON_32: Asset = asset!("assets/icons/favicon-32x32.png");
const APPLE_TOUCH_ICON: Asset = asset!("assets/icons/apple-touch-icon.png");
const ANDROID_ICON_192: Asset = asset!("assets/icons/android-chrome-192x192.png");
const ANDROID_ICON_512: Asset = asset!("assets/icons/android-chrome-512x512.png");
const MASKABLE_ICON_512: Asset = asset!("assets/icons/maskable-icon-512x512.png");

/// Builds the application's router.
pub fn router() -> RouterBuilder {
    module_router!()
        .font(DISPLAY_FONT)
        .font(BODY_FONT)
        .assets(AssetBundle::load().unwrap())
        .route(web_manifest)
        // The live preview of the birth form.
        .route(menh_preview)
        // Reads and writes the language cookie.
        .layer(CookieLayer::new())
        // Lets the browser runtime rerun pages; register it after other layers.
        .layer(RuntimeLayer::new())
}

/// `GET /manifest.webmanifest`, for "Add to home screen". It is built per
/// request because the icon URLs carry a content hash.
#[route(GET "/manifest.webmanifest")]
async fn web_manifest(cx: &Cx) -> Result<Json<serde_json::Value>> {
    let icon = |asset, sizes, purpose| json!({ "src": asset_config(cx).resolve(cx, asset), "sizes": sizes, "type": "image/png", "purpose": purpose });
    Ok(Json(json!({
        "name": "Thiên Hạc",
        "short_name": "Thiên Hạc",
        "description": "Thiên Hạc: draw a Tử Vi card and plan your day",
        "start_url": "/",
        "display": "standalone",
        // A manifest cannot read CSS variables: these are the `page` and
        // `bg-0` tokens of styles.css.
        "background_color": "#120a24",
        "theme_color": "#1a1033",
        "icons": [
            icon(ANDROID_ICON_192, "192x192", "any"),
            icon(ANDROID_ICON_512, "512x512", "any"),
            icon(MASKABLE_ICON_512, "512x512", "maskable"),
        ],
    })))
}

/// Puts the request's locale in the context, before any page renders.
#[layer]
async fn locale_layer(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let locale = request_locale(cx);
    next.run(&cx.with(locale), body).await
}

#[layout]
async fn root(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    let lang = Lang::of(cx);
    let theme = Theme::of(cx);
    Ok(view! {
        <!DOCTYPE html>
        <html lang=(lang.code()) class=(theme.code())>
            <head>
                <meta charset="utf-8">
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, viewport-fit=cover"
                >
                <meta name="theme-color" content=(theme.bar_color())>
                <title>"Thiên Hạc · " (t!(meta_title))</title>
                <meta
                    name="description"
                    content=(t!(meta_description).resolve(cx))
                >
                <link rel="icon" href=(FAVICON) sizes="any">
                <link rel="icon" type="image/png" sizes="16x16" href=(FAVICON_16)>
                <link rel="icon" type="image/png" sizes="32x32" href=(FAVICON_32)>
                <link rel="icon" type="image/png" sizes="48x48" href=(LOGO)>
                <link rel="apple-touch-icon" sizes="180x180" href=(APPLE_TOUCH_ICON)>
                <link rel="manifest" href="/manifest.webmanifest">
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
                topcoat::font::link(font: DISPLAY_FONT)
                topcoat::font::link(font: BODY_FONT)
                topcoat::runtime::script()
                topcoat::dev::script()
            </head>
            <body>
                cosmos()
                <div class="relative z-10">
                    <header
                        class="mx-auto flex max-w-5xl items-center justify-between gap-3 px-4 pt-6 pb-5"
                    >
                        brand()
                        <div class="flex items-center gap-2">
                            language_menu(lang: lang)
                            theme_toggle(theme: theme)
                        </div>
                    </header>
                    (slot)
                </div>
            </body>
        </html>
    })
}

/// The language menu: a language icon that opens a dropdown with every
/// language; the page's own is checked. The dropdown is an HTML `popover`, so
/// the browser opens and closes it without a script. Each choice posts to
/// `/lang` and comes back to this page.
#[topcoat::view::component]
async fn language_menu(cx: &Cx, lang: Lang) -> Result<impl View> {
    let back = uri(cx)
        .path_and_query()
        .map_or("/", |path| path.as_str())
        .to_owned();
    let action = href!(lang::switch).resolve(cx);
    Ok(view! {
        <div class="relative">
            <button
                type="button"
                popovertarget="language-menu"
                aria-label=(t!(lang_menu_aria).resolve(cx))
                style="anchor-name: --language-button"
                class="pill-ghost inline-flex h-9 cursor-pointer items-center gap-1.5 rounded-full px-3 text-xs font-semibold"
            >
                <span class="text-base text-accent">icon(data: iconify_icon!("lucide:languages"))</span>
                (lang.pick("En", "Vi"))
            </button>
            <div
                id="language-menu"
                popover="auto"
                class="language-menu glass m-0 min-w-44 p-1.5 text-sm"
            >
                for option in [Lang::En, Lang::Vi] {
                    <form method="post" action=(action.as_str())>
                        <input type="hidden" name="lang" value=(option.code())>
                        <input type="hidden" name="back" value=(back.as_str())>
                        <button
                            type="submit"
                            lang=(option.code())
                            aria-current=((option == lang).then_some("true"))
                            class=(class!(
                                "flex w-full cursor-pointer items-center justify-between gap-3 rounded-[14px] px-3 py-2 text-left transition-colors hover:bg-white/8",
                                "font-semibold text-accent" if option == lang,
                            ))
                        >
                            <span>
                                <b class="mr-2 inline-block w-5 font-semibold">(option.pick("En", "Vi"))</b>
                                <span class=(if option == lang { "" } else { "text-muted" })>
                                    (option.pick("English", "Tiếng Việt"))
                                </span>
                            </span>
                            if option == lang {
                                <span aria-hidden="true">"✓"</span>
                            }
                        </button>
                    </form>
                }
            </div>
        </div>
    })
}

/// The fixed backdrop: twilight gradient and two slow nebulae. The nebulae
/// are hidden on phones, where the content fills the screen.
#[topcoat::view::component]
async fn cosmos(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <div aria-hidden="true" class="cosmos fixed inset-0 z-0 overflow-hidden">
            <div
                class="nebula animate-breathe absolute top-[-15vmax] left-[-10vmax] h-[60vmax] w-[60vmax] rounded-full bg-[radial-gradient(circle,rgba(123,79,166,0.6),transparent_60%)] opacity-50 blur-[60px] max-sm:hidden"
            ></div>
            <div
                class="nebula animate-breathe absolute right-[-12vmax] bottom-[-10vmax] h-[50vmax] w-[50vmax] rounded-full bg-[radial-gradient(circle,rgba(79,157,224,0.35),transparent_60%)] opacity-50 blur-[60px] [animation-delay:-4s] max-sm:hidden"
            ></div>
        </div>
    })
}

#[page]
pub async fn home(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <main class="mx-auto max-w-5xl px-4 pb-16">cast_screen(sent: None, error: None)</main>
    })
}
