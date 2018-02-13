extern crate protobuf;
extern crate zmq;
use zmq::{Context};
use std::error::Error;

mod protos;

// Broker subscribes to client and will make a request from to a worker
// it will send the message to the worker
fn run_broker(ctx: &mut Context, addr: &str) -> Result<(), Box<Error>> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;

    let received_message: i32 = Default::default();


	// let mut send_msg: Vec<u8> = Vec::new();
	let send_msg: Vec<u8> = sock.recv_bytes(received_message)?;
    println!("Message received!");
	// we got the message, now call the worker
	let sock2 = ctx.socket(zmq::REQ)?;
	let addr2 = "tcp://127.0.0.1:25932";
	sock2.connect(addr2)?;
    // println!("-> {:?}", client_req);
    sock2.send(&send_msg, 0)?;
    println!("Message sent to worker!");

    // received the message from the worker
    let received_message2: i32 = Default::default();
    // let mut worker_res: Vec<u8> = Vec::new();
    let worker_res: Vec<u8> = sock2.recv_bytes(received_message2)?;
    println!("Message received from worker!");
    // send message back to the client
    sock.send(&worker_res, 0)?;
    println!("Message sent to client!");

    Ok(())
		
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    run_broker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}