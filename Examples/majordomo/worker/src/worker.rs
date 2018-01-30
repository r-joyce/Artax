extern crate zmq;
extern crate protobuf;

mod protos;

use zmq::{Context, Error};


// client is the subscriber, it will send requests to the client
fn run_worker(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;

    let received_message: i32 = Default::default();

    let send_msg: Vec<u8> = sock.recv_bytes(received_message)?;
    println!("Message received!");

    // send message back to the client
    sock.send(&send_msg, 0)?;
    println!("Message sent back to broker!");

    Ok(())

}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25932";
    run_worker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}

