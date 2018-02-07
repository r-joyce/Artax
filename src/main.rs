extern crate protobuf;
extern crate snap;
extern crate zmq;

use zmq::{Context, Error, Message};

// server is the publisher, it must never receive anything
fn run_server(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REQ)?;
    sock.connect(addr)?;
    let payload = "Hello world!";
    println!("-> {:?}", payload);
    //let mut msg = Message::new()?;
    let mut writer = snap::Encoder::new();
    let compressed_message = writer.compress_vec(&payload.as_bytes()).unwrap_or_default();
    println!("{:?}", compressed_message);
    sock.send(compressed_message, 0)?;

    //sock.recv(&mut msg, 0)?;
    //let contents = msg.as_str().unwrap();
    //println!("<- {:?}", contents);
    Ok(())
}

// client is the subscriber, it must never send anything
fn run_client(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;
    let mut msg = Message::new();
    let mut reader = snap::Decoder::new();
    loop {
        if sock.recv(&mut msg, 0).is_ok() {
            let compressed_message = reader
                .decompress_vec(&msg.as_str().unwrap_or_default().as_bytes())
                .unwrap_or_default();
            println!("{:?}", String::from_utf8(compressed_message).unwrap());
            //sock.send(msg.as_str().unwrap().as_bytes(), 0)?;
            println!("Message received!");
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Usage: {} (client|server)", args[0]);
        return;
    }
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    if args[1] == "server" {
        println!("ZeroMQ server connecting to {}", addr);
        //while true {
        run_server(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
    //}
    } else {
        println!("ZeroMQ client listening on {}", addr);
        //while true {
        run_client(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
        //}
    }
}
