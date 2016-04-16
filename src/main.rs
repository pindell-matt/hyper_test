extern crate hyper;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let args: Vec<_> = env::args().collect();

    let url: String = args[1].to_owned();
    let url_slice: &str = &url[..];

    let mut client = Client::new();

    let mut res = client.get(url_slice)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}", body);
}
