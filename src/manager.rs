// use std::io::{self, Write};
// use futures::{Future, Stream};
use reqwest::{self, Client, Url};
// use hyper::{Client};
// use tokio_core::reactor::Core;

// fn body_to_string(res: hyper::Response) -> Box<Future<Item=String, Error=hyper::error::Error>> {
//     let body = res.body().concat2().map(| x | {
//         match String::from_utf8(x.to_vec()) {
//             Ok(body) => body,
//             Err(_x) => String::from("")
//         }
//     });
//     Box::new(body)
// }

pub struct RequestManager {
  client: Client,
  base_uri: String,
  group_id: String,
  token: String,
}

impl RequestManager {
  pub fn new(base_uri: String, group_id: String, token: String) -> RequestManager {
    let client = Client::new();
    RequestManager { client , base_uri, group_id, token }
  }

  // pub fn get(&mut self, uri: String) -> Result<String, hyper::Error> {
  //   let url = uri.parse::<hyper::Uri>().unwrap();
  //   let client = Client::new(&self.core.handle());
  //   let task = client.get(url).and_then(body_to_string);
  //   self.core.run(task)
  // }

  pub fn get_posts(&mut self) -> Result<String, reqwest::Error> {
    let uri_string = format!("{}/{}/feed?access_token={}", self.base_uri, self.group_id, self.token);
    let uri = Url::parse(&uri_string).unwrap();
    let mut resp = self.client.get(uri).send()?;
    let body = resp.text()?;
    Ok(body)
    // let uri = uri_string.parse::<hyper::Uri>().unwrap();
    // let client = Client::new(&self.core.handle());
    // let task = client.get(uri).and_then(body_to_string);
    // self.core.run(task)
  }
}