#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use slog::Fuse;

fn main() {
    let log = slog::Logger::root(slog_term::streamer().stderr().build().fused(), o!("version" => "0.5"));
    slog_stdlog::set_logger(log.clone()).unwrap();

    info!("standard logging redirected to slog");
}
