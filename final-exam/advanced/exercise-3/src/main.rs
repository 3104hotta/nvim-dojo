// Time-attack: implement the unfilled functions until `cargo build` passes.
//
// Required:
//   - Config::new(host, port) and Display impl
//   - Router::add_route, dispatch
//   - Handler trait: handle_get, handle_post (default impls or impl on a struct)
//   - main() that wires Config + Router + Handler together

mod config;
mod handler;
mod router;

use crate::config::Config;
use crate::handler::Handler;
use crate::router::Router;

fn main() {
    // Implement: build a Config, register handlers in a Router, dispatch a sample request.
    todo!()
}
