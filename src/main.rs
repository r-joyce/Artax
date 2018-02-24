extern crate protobuf;
extern crate zmq;
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
use reduction::*;

const THREADS: usize = 4;

// Broker subscribes to client and will make a request from to a worker
// it will send the message to the worker
fn run_broker(ctx: &mut Context, addr: &str) -> Result<(), Box<Error>> {

    let sock = ctx.socket(zmq::REP)?;
    sock.bind(addr)?;

    loop {
        // Receive algorithm string and data
        let received_message: i32 = Default::default();
        let job = sock.recv_string(0).unwrap().unwrap();
    	let send_msg: Vec<u8> = sock.recv_bytes(received_message)?;
        println!("Message received!");

        //Decompress data
        let mut reader = snap::Decoder::new();
        let compressed_data = reader.decompress_vec(&send_msg).unwrap();
        let mut data = message::Message::new();
        data.merge_from_bytes(&compressed_data).unwrap();

    	// Distribute job to workers
        let results = worker_task(job, data);

        // Package results message to send back to client
        let mut new_data = message::ReductionMessage::new();
        let mut new_min = message::ReductionMessage_Min::new();
        let mut new_max = message::ReductionMessage_Max::new();
        new_data.set_sum(results.sum);
        new_data.set_avg(results.avg);
        new_min.set_min_x(results.min.0);
        new_min.set_min_y(results.min.1);
        new_data.set_min(new_min);
        new_max.set_max_x(results.max.0);
        new_max.set_max_y(results.max.1);
        new_data.set_max(new_max);
        let mut writer = snap::Encoder::new();
        let recompressed_data = writer.compress_vec(&new_data.write_to_bytes().unwrap()).unwrap();

        // send message back to the client
        sock.send(&recompressed_data, 0)?;
        println!("Message sent to client!");
        println!();
    }

    Ok(())
}

// Distribute task to worker threads
fn worker_task(job: String, mut data: message::Message) -> reduction::Reduction {

    match job.as_str() {
        "Reduction" => {
            // Create a new vector containing the tic values
            let mz_values = Arc::new(data.take_mz());
            let tic_values = Arc::new(data.take_tic());
            let mut handles = vec![];
            let (tx, rx): (Sender<Values>, Receiver<Values>) = mpsc::channel();

            // Perform calculations using threads, pass data to receiver using channel
            for _thread in 0..THREADS {
                let thread_tx = tx.clone();
                let mz_values_clone = Arc::clone(&mz_values);
                let tic_values_clone = Arc::clone(&tic_values);
                let handle = thread::spawn(move || {
                    if _thread == 0 {
                        thread_tx.send(reduction::calc_sum(tic_values_clone.to_vec())).expect("should be sum");
                    }
                    else if _thread == 1 {
                        thread_tx.send(reduction::calc_max(mz_values_clone.to_vec(), tic_values_clone.to_vec())).expect("should be max");
                    }
                    else if _thread == 2 {
                        thread_tx.send(reduction::calc_min(mz_values_clone.to_vec(), tic_values_clone.to_vec())).expect("should be min");
                    }
                    else {
                        thread_tx.send(reduction::calc_avg(tic_values_clone.to_vec())).expect("should be avg");
                    }
                });
                handles.push(handle);
            }

            // Create a results vector and insert values from receiver channel into it
            // Order: [ avg, sum, min, max ]
            let mut results = Reduction::new();

            for _ in 0..THREADS {
                let temp_data = rx.recv().unwrap();
                match temp_data.calc {
                    1 => results.sum = temp_data.value,
                    2 => results.avg = temp_data.value,
                    3 => {
                            results.min.0 = temp_data.point.x;
                            results.min.1 = temp_data.point.y
                         }
                    4 => {
                            results.max.0 = temp_data.point.x;
                            results.max.1 = temp_data.point.y
                         }
                    _ => println!("Unexpected calculation returned!"),
                }
            }

            println!("Sum: {:?}\nAvg: {:?}\nMin: [{:?}, {:?}]\nMax: [{:?}, {:?}]",
                    results.sum, results.avg, results.min.0, results.min.1, results.max.0, results.max.1);

            // Join child threads so main can continue
            for handle in handles {
                handle.join().unwrap();
            }

            println!("Reduction complete!");

            results
        }
        _ => {
            let results = Reduction::new();
            results
        }
        /*"Thrash" => {

            },
        "Msms" => {

        },
        "Resolution" => {

        },
        */
    }
}

fn main() {
    let mut ctx = Context::new();
    let addr = "tcp://127.0.0.1:25933";
    run_broker(&mut ctx, addr).unwrap_or_else(|err| println!("{:?}", err));
}
