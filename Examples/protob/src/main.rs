extern crate protobuf;

mod protos;
use protos::message;
use std::io::{self, BufRead};

#[test]
fn derp() {
}

fn main() {
	// Get input
	print!("Encode: ");
	io::Write::flush(&mut io::stdout()).expect("flush failed!");

	let reader = io::stdin();
	let nums: Vec<i32> =
		reader.lock()
			.lines().next().unwrap().unwrap()
			.trim().split(' ')
        	.map(|s| s.parse().unwrap())
        	.collect();
	
	// Show raw input
	println!("Plain: {:?}", nums);

	// Encode
	let mut message = message::Message::new();
	message.set_mz(nums);

	// TODO: Is it really encoded?
	// Show encoded message
	println!("Encoded: {:?}", message.get_mz());
}