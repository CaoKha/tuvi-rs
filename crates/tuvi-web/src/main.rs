mod app;
mod features;

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(error) = topcoat::start(app::router().build()).await {
        eprintln!("{error:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
