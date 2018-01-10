extern crate zmq;
extern crate protobuf;
extern crate bincode;
//extern crate chrono;
//extern crate serde;
//#[macro_use] extern crate serde_derive;

mod protos;
//use protos::message;
//use std::io::{self, BufRead};
use zmq::{Context, Message, Error};
//use chrono::NaiveDateTime;
//use serde::{Serialize, Serializer, Deserialize, Deserializer};

// Broker subscribes to client and will make a request from to a worker
// it will send the message to the worker
fn run_broker(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;
    let mut msg = Message::new()?;
    loop {
        if sock.recv(&mut msg, 0).is_ok() {
			println!("Message received!");
			// we got the message, now call the worker
			let sock2 = ctx.socket(zmq::REQ)?;
			let addr2 = "tcp://127.0.0.1:25932";
			sock2.connect(addr2)?;
			//let client_req = msg.as_str().unwrap();
			let client_req: Vec<i32> = bincode::serde::deserialize(&msg).unwrap();
		    println!("-> {:?}", client_req);
		    let mut worker_res = Message::new()?;
			let client_req_serialized = bincode::serde::serialize(&client_req, bincode::SizeLimit::Infinite).unwrap();
		    //sock2.send(client_req.as_bytes(), 0)?;
			sock2.send(&client_req_serialized, 0)?;
			sock2.recv(&mut worker_res, 0)?;
			let decoded_msg: Vec<i32> = bincode::serde::deserialize(&worker_res).unwrap();
			//let res_contents = worker_res.as_str().unwrap();
			println!("<- {:?}", decoded_msg);
			let worker_res_serialized = bincode::serde::serialize(&decoded_msg, bincode::SizeLimit::Infinite).unwrap();
            sock.send(&worker_res_serialized, 0)?;
		}
    }
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    run_broker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}