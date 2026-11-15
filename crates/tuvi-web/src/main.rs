mod app;
mod features;
mod i18n;

use std::process::ExitCode;

// The interface text: `t!` and `t::locales()` (see `i18n.rs`).
topcoat::l10n::fluent::messages!("locales/{locale}.ftl");

#[tokio::main]
async fn main() -> ExitCode {
    let guide = match features::today::Guide::load() {
        Ok(guide) => guide,
        Err(error) => {
            eprintln!("cannot read data/daily_guide.json: {error}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(error) = topcoat::start(app::router().with(guide).build()).await {
        eprintln!("{error:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
