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
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

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
    let mut data = message::Message::new();
    data.merge_from_bytes(&compressed_data).unwrap();

    // Create a new vector containing the tic values
    let values = Arc::new(data.take_tic());
    let mut handles = vec![];
    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();

    // Perform calculations using threads, pass data to receiver using channel
    for _thread in 0..THREADS {
        let thread_tx = tx.clone();
        let values_clone = Arc::clone(&values);
        let handle = thread::spawn(move || {
            if _thread == 0 {
                thread_tx.send(reduction::calc_sum(values_clone.to_vec())).expect("should be sum");
            }
            else if _thread == 1 {
                thread_tx.send(reduction::calc_max(values_clone.to_vec())).expect("should be max");
            }
            else if _thread == 2 {
                thread_tx.send(reduction::calc_min(values_clone.to_vec())).expect("should be min");
            }
            else {
                thread_tx.send(reduction::calc_avg(values_clone.to_vec())).expect("should be avg");
            }
        });
        handles.push(handle);
    }

    // Create a results vector and push values from receiver channel into it
    let mut results = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        results.push(rx.recv().unwrap());
    }

    print!("Results: [ ");
    for result in results.iter() {
        print!("{:?} ", result);
    }
    println!("]");

    // Join child threads so main can continue
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Reduction complete!");

    // Create message to send back to broker, populate tic vector with results
    let mut new_data = message::Message::new();
    new_data.set_tic(results);

    // Compress and encode vector into bytes to be sent
    let mut writer = snap::Encoder::new();
    let recompressed_data = writer.compress_vec(&new_data.write_to_bytes().unwrap()).unwrap();

    // send message back to the client
    sock.send(&recompressed_data, 0)?;
    println!("Message sent back to broker!");

    Ok(())

}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25932";
    run_worker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}
