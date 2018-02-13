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
use reduction::Values;

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
        let mut new_data = message::Message::new();
        new_data.set_tic(results);
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
fn worker_task(job: String, mut data: message::Message) -> Vec<u32> {

    match job.as_str() {
        "Reduction" => {
            // Create a new vector containing the tic values
            let values = Arc::new(data.take_tic());
            let mut handles = vec![];
            let (tx, rx): (Sender<Values>, Receiver<Values>) = mpsc::channel();

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

            // Create a results vector and insert values from receiver channel into it
            // Order: [ avg, sum, min, max ]
            let mut results: Vec<u32> = vec![0; 4];

            for _ in 0..THREADS {
                let temp_data = rx.recv().unwrap();
                match temp_data.calc {
                    1 => results[0] = temp_data.value,
                    2 => results[1] = temp_data.value,
                    3 => results[2] = temp_data.value,
                    4 => results[3] = temp_data.value,
                    _ => println!("Unexpected calculation returned!"),
                }
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

            results
        }
        _ => {
            let results = vec![0; 4];
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
