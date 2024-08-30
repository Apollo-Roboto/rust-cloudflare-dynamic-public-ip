mod cli;
mod cloudflare;
mod cloudflare_client;
mod logger;

use logger::LOGGER;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
    std::process::exit(cli::run().await);
}
