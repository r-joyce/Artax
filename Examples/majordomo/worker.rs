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

// client is the subscriber, it will send requests to the client
fn run_worker(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;
    let mut msg = Message::new()?;
    loop {
        if sock.recv(&mut msg, 0).is_ok() {
			// depending on the message, the worker will send a certain response
			let client_req: Vec<i32> = bincode::serde::deserialize(&msg).unwrap();
			let worker_res_serialized = bincode::serde::serialize(&client_req, bincode::SizeLimit::Infinite).unwrap();
            sock.send(&worker_res_serialized, 0)?;
			println!("Worker got the message!");
        }
    }

}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25932";
    run_worker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}

