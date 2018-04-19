extern crate iron;

use std::env;
use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Hello Rust + Apex Up!")))
    }
    let port = env::var("PORT").unwrap();
    let _server = Iron::new(hello_world)
        .http(format!("localhost:{}", port))
        .unwrap();
    println!("On {}", port);
}
