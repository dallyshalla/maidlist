extern crate hyper;
extern crate rustc_serialize;

use std::io;
use std::io::Read;

use rustc_serialize::{Decoder};
use rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::Connection;

#[derive(RustcDecodable, RustcEncodable)]
struct SingleData {
    balance: i32,
    reserved_balance: i32,
    address: String,
}

fn main() {
		let client = Client::new();


        let mut results = client.get("https://www.omniwallet.org/v1/mastercoin_verify/addresses?currency_id=3")
        .header(Connection::close())
        .send().unwrap();

        let mut payload = String::new();
        results.read_to_string(&mut payload).unwrap();

        println!("type in a minimum maidsafecoin balance this will read back to you the balance");
        let mut input_text = String::new();
    	io::stdin()
        	.read_line(&mut input_text)
        	.expect("failed to read from stdin");
		let trimmed = input_text.trim();

		let mut new_int = 0;
		match trimmed.parse::<i32>() {
            Ok(i) => new_int = i,
            Err(..) => println!("this was not an integer: {}", trimmed)
        };

    	let mut int_new = 0;
        let decoded: Vec<SingleData> = json::decode(&payload).unwrap();
        for thethings in 0..decoded.len() {
        	
            if decoded[thethings].balance > new_int {
            	int_new += 1;
                println!("#{:?}", int_new);

                println!("address: {:?}", &decoded[thethings].address);
                println!("maidsafe coin balance: {:?}", &decoded[thethings].balance);
            }
        }
}