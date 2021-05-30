use http::request::Request;
use http::response::Response;
use server::Server;

mod http;
mod server;

fn main() {
    /*let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    println!("{:?}, {:?}", action, item);*/
    let server = Server::new("127.0.0.1:3000".to_string());
    server.run();
}
