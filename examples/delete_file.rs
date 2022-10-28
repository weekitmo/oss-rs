//! `cargo run --example delete_file --features=blocking`

use aliyun_oss_client::client::Client;
use aliyun_oss_client::blocking::builder::ClientWithMiddleware;

extern crate dotenv;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let client = Client::<ClientWithMiddleware>::from_env().unwrap();
    //let headers = None;
    client.delete_object("examples/bg2015071010.png").unwrap();
    println!("delet file success");
}
