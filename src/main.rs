extern crate futures;
extern crate hyper;
extern crate tokio_core;

mod manager;
use manager::Manager;

fn main() {
    let mut manager = Manager::new();
    let uri = String::from("http://jsonplaceholder.typicode.com/comments?postId=1&&id=1");
    match manager.get(uri.clone()) {
        Ok(resp) => println!("1 {}", resp),
        Err(_e) => println!("{:?}", _e)
    };

    match manager.get(uri.clone()) {
        Ok(resp) => println!("2 {}", resp),
        Err(_e) => println!("{:?}", _e)
    };
}