//! ch-final intermediate-1
//! migration target: log -> tracing

mod handler;
mod middleware;
mod router;
mod store;

use log::info;

fn main() {
    log::info!("starting app");
    info!("after init");
}
