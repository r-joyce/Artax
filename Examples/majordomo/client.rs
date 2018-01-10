extern crate zmq;
extern crate protobuf;
//extern crate bincode;
extern crate lzf;
//extern crate chrono;
//extern crate serde;
//#[macro_use] extern crate serde_derive;

mod protos;
use protos::message;
//use protobuf::Message;
use protobuf::Message;
use protobuf::parse_from_bytes;
use std::io::{self, BufRead};
use zmq::{Context, Error};
//use chrono::NaiveDateTime;
//use serde::{Serialize, Serializer, Deserialize, Deserializer};

// Client sends a request to a broker
fn run_client(ctx: &mut Context, addr: &str) -> Result<(), Error> {
	let sock = ctx.socket(zmq::REQ)?;
    sock.connect(addr)?;

	loop {
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

//		let payload = bincode::serde::serialize(&nums, bincode::SizeLimit::Infinite).unwrap();
		println!("-> {:?}", nums);

		// Encoded message
		let mut msg = message::Message::new();
		msg.set_mz(nums);

		// Compress
		let results = msg.write_to_bytes();
		let comp = lzf::compress(&results.unwrap());
		println!("Encoded: {:?}", comp);

		let payload = comp.unwrap();

		sock.send(&payload, 0)?;
		//sock.recv(&mut msg, 0)?;

		// TODO won't be able to decode until we figure out 
		// how to decompress the message
/*		let decoded_msg: Vec<i32> = bincode::serde::deserialize(&msg).unwrap();
		//let contents = decodedMsg.as_str().unwrap();
		println!("<- {:?}", decoded_msg);*/
	}

//    Ok(())
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
	println!("ZeroMQ server connecting to {}", addr);
	run_client(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}