extern crate zmq;

use zmq::{Context, Message, Error};

// client is the subscriber, it must never send anything
fn run_client(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;
    let mut msg = Message::new()?;
    loop {
        if sock.recv(&mut msg, 0).is_ok() {
            //sock.send(msg.as_str().unwrap().as_bytes(), 0)?;
			println!("Message received!");
        }
    }
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    run_client(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}

