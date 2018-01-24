extern crate futures;
extern crate hyper;
extern crate tokio_core;

// use std::io::{self, Write};
use futures::{Future, Stream, future};
use hyper::Client;
use hyper::client::HttpConnector;
use tokio_core::reactor::Core;

fn body_to_string(res: hyper::Response) -> Box<Future<Item=String, Error=hyper::error::Error>> {
    let body = res.body().concat2().map(| x | {
        match String::from_utf8(x.to_vec()) {
            Ok(body) => body,
            Err(_x) => String::from("")
        }
    });
    Box::new(body)
}

fn get(uri: String, client: hyper::Client<HttpConnector>) -> Box<Future<Item=String, Error=hyper::error::Error>>
{
    let url = uri.parse::<hyper::Uri>().unwrap();
    Box::new(client.get(url).and_then(body_to_string))
}

fn main() {
    let mut core = Core::new().unwrap();

    let task1 = get(String::from("http://httpbin.org/ip"), Client::new(&core.handle()));
    core.run(task1).and_then(| resp | {
        println!("{}", resp);
        Ok(())
    });
    
    let task2 = get(String::from("http://httpbin.org/ip"), Client::new(&core.handle()));
    core.run(task2).and_then(| resp | {
        println!("{}", resp);
        Ok(())
    });
    ()
    // let resp = core.run(task).unwrap();
    // println!("{}", resp);
}