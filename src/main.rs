#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate prometheus;

use env_logger::Env;
use log::{debug, error};
use structopt::StructOpt;

mod consumer;
mod metrics;

#[derive(StructOpt, Debug)]
#[structopt(name = "metrics-from-status")]
struct Opt {
    /// Activate debug mode (TODO: actually emit some stuff)
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// TTY from which to read data
    #[structopt(short = "t", long = "tty", default_value = "/dev/ttyACM0")]
    tty: String,
}

#[actix_rt::main]
async fn main() {
    let opt = Opt::from_args();
    let level = if opt.debug { "debug" } else { "info" };

    // FIXME: logic is reversed here, commandline should override environment
    env_logger::from_env(
        Env::default()
        .default_filter_or(level)
    )
    .init();

    // TODO: make this an actor too...
    let _handle = std::thread::spawn(|| {
        let tty = opt.tty;

        debug!("Starting reader.");
        if let Err(e) =
            consumer::consume_data(tty)
        {
            error!("Data reader failed: {}", e);
        }
    });

    if let Err(e) = metrics::server().await {
        error!("Metrics server failed: {}", e);
    }
}
