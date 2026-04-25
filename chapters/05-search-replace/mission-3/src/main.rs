mod server;
mod client;
mod config;

fn main() {
    let cfg = config::load();
    let srv = server::Server::new(&cfg);
    println!("server: {:?}", srv);
}
