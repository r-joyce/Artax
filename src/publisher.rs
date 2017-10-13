extern crate zmq;

use zmq::{Context, Error};

// server is the publisher, it must never receive anything
fn run_server(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REQ)?;
    sock.connect(addr)?;
    let payload = "Hello world!";
    println!("-> {:?}", payload);
    //let mut msg = Message::new()?;
    sock.send(payload.as_bytes(), 0)?;
    Ok(())
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
	println!("ZeroMQ server connecting to {}", addr);
	run_server(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}