pub mod handler;
pub mod router;
pub mod server;

use server::Server;
fn main() {
    let s = Server::new("127.0.0.1:2222");
    s.run();
}
