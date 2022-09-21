use http::*;

fn main() {
    let port = 80;

    let mut server = Server::bind(port).expect(&format!("server failed to start on port {}", port));

    loop {
        server.respond();
    }
}
