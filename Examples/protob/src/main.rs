extern crate protobuf;
extern crate lzf;

mod protos;
use protos::message;
use protobuf::Message;
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
    let length = nums.len();
	
	// Show raw input
	println!("Plain: {:?}", nums);

	// Encode
	let mut message = message::Message::new();
	message.set_mz(nums);

	// Compress
	let results = message.write_to_bytes();
	let comp = lzf::compress(&results.unwrap());
	println!("Encoded: {:?}", comp);

	// Decompress
	let decomp = lzf::decompress(&comp.unwrap(), length);

	// Show encoded message
	println!("Decoded: {:?}", decomp);
}