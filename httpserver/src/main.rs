use crate::server::Server;

mod server;
mod handler;
mod router;

fn main() {
    let server = Server::new("localhost:3000");
    server.run();
}
