extern crate hyper;
extern crate url;
extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

use url::form_urlencoded;
use std::io::Read;
use hyper::{Client};

fn get_content(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.get(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

fn post_query(url: &str) -> hyper::Result<String> {
    let client = Client::new();
    let mut response = try!(client.post(url).send());
    let mut buf = String::new();
    try!(response.read_to_string(&mut buf));
    Ok(buf)
}

fn main() {
    // println!("{:?}", post_query("http://universe.rejs.io/api/v1/data").unwrap());
    // println!("{:?}", get_content("http://universe.rejs.io/api/v1/data").unwrap());

    let new_request = get_content("http://universe.rejs.io/api/v1/data").unwrap();
    if let Ok(request_json) = json::Json::from_str(&new_request) {
        for i in 0..50 {
            if let Some(ref atom) = request_json[i].find("atom") {
                if let Some(ref electrons) = atom.find("electrons") {
                    println!("Electrons: {}", electrons);
                }
            }
        }
    }
}
