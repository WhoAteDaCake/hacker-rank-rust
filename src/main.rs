extern crate futures;
extern crate reqwest;
extern crate tokio_core;

mod manager;
use manager::RequestManager;

static TOKEN: &'static str ="EAAMMXECigFsBAFaJP3af4VvUqZBkVVCrRZAATZCqQ8pOPu37reum9eLCOsGskLnmZBvWS2MDKoy2jKBmluTJoH9ZBZBCLtycfrPGVtH69hbvlplnmEl3ssh8ykasEZCVeXxjFeCklWmkyzIUEPmWchbYZAgaX5yMZCjM6OEK7HxFAiwZDZD";
static BASE_URI: &'static str = "https://graph.facebook.com/v2.11";
static GROUP_ID: &'static str = "352110954925626";
/*
    Graph base url
    https://graph.facebook.com/v2.11

    https://www.facebook.com/groups/352110954925626/permalink/1150443481759032
    For single post
*/
fn main() {
    let mut manager = RequestManager::new(
        String::from(BASE_URI),
        String::from(GROUP_ID),
        String::from(TOKEN),
    );
    // manager.get_posts();
    // let uri = String::from("http://jsonplaceholder.typicode.com/comments?postId=1&&id=1");
    match manager.get_posts() {
        Ok(resp) => println!("1 {}", resp),
        Err(_e) => println!("{:?}", _e)
    };
    // println!("{}", manager.get_posts());
    // match manager.get(uri.clone()) {
    //     Ok(resp) => println!("2 {}", resp),
    //     Err(_e) => println!("{:?}", _e)
    // };
}