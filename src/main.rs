mod cli;
mod cloudflare;
mod logger;

use logger::LOGGER;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    log::set_logger(&LOGGER).unwrap();
    std::process::exit(cli::run().await);
}
