extern crate zmq;
extern crate protobuf;
extern crate snap;

mod protos;
mod reduction;

use zmq::{Context, Error};
use std::thread;
use protobuf::Message;
use protos::message;
use std::sync::Arc;

const THREADS: usize = 4;

// client is the subscriber, it will send requests to the client
fn run_worker(ctx: &mut Context, addr: &str) -> Result<(), Error> {
    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;

    let received_message: i32 = Default::default();

    let send_msg: Vec<u8> = sock.recv_bytes(received_message)?;
    println!("Message received!");

    // extract and perform reduction
    let mut reader = snap::Decoder::new();
    let compressed_data = reader.decompress_vec(&send_msg).unwrap();


//     fn merge_from_bytes(&mut self, bytes: &[u8]) -> ProtobufResult<()>
// fn merge_from(&mut self, is: &mut CodedInputStream) -> ProtobufResult<()>

    let mut data = message::Message::new();
    data.merge_from_bytes(&compressed_data).unwrap();
    let values = Arc::new(data.take_tic());
    let mut handles = vec![];

    // print!("Vector: [ ");
    // for i in values.iter() {
    //     print!("{:?} ", i);
    // }
    // println!("]");



    for _thread in 0..THREADS {
        let values_clone = Arc::clone(&values);
        let handle = thread::spawn(move || {
            if _thread == 0 {
                reduction::calc_sum(_thread, values_clone.to_vec());
            }
            else if _thread == 1 {
                reduction::calc_max(_thread, values_clone.to_vec());
            }
            else if _thread == 2 {
                reduction::calc_min(_thread, values_clone.to_vec());
            }
            else {
                reduction::calc_avg(_thread, values_clone.to_vec());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Reduction complete!");

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
