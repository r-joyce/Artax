extern crate zmq;
extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;

mod protos;
mod reduction;

use protos::message;
use zmq::Context;
use std::error::Error;
use std::process;
use std::env;
use protobuf::Message;
use reduction::*;


fn main() {
    println!("[+] Initializing client...");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            // Obtain analysis to run
            let algorithm = &args[1].to_lowercase();

            // Connect Falkor (REQ) to Artax (ROUTER)
            let ctx = Context::new();
		    let router_addr = "tcp://127.0.0.1:25933";
            let request = ctx.socket(zmq::REQ).unwrap();
            request.connect(router_addr).unwrap();

            // Connect Falkor (SUB) to Artax (PUB)
            //let pub_addr = "tcp://127.0.0.1:25931";
            //let subscriber = ctx.socket(zmq::SUB).unwrap();
            //subscriber
            //    .connect(pub_addr)
            //    .expect("failed connecting subscriber");
            //subscriber
            //    .set_subscribe(algorithm.as_bytes())
            //    .expect("failed subscribing");

            println!("[+] Begin receiving analysis...");

            // Run Falkor simulation
            if let Err(err) = run_client(algorithm.to_string(), request) {
                println!("[!] Error running client: {}", err);
                process::exit(1);
            }
        },
        _ => {
            help();
        }
    }
}


// Compress and send data
fn run_client(algorithm: String, request: zmq::Socket) -> Result<(), Box<Error>> {

    // Send analysis request, receive and print results
    match algorithm.as_ref() {
        "reduction" => {

            loop {
                request.send("reduction", 0).unwrap();
                // Receive data on subscriber
                let new_message: Vec<u8> = request.recv_bytes(0).unwrap();

                // decompress the data
                let mut reader = snap::Decoder::new();
                let uncompressed_data = reader.decompress_vec(&new_message).unwrap();

                let mut results = message::ReductionMessage::new();
                results.merge_from_bytes(&uncompressed_data).unwrap();

                let mut reduction = Reduction::new();
                reduction.sum = results.get_sum();
                reduction.avg = results.get_avg();
                reduction.min.0 = results.get_min().get_min_x();
                reduction.min.1 = results.get_min().get_min_y();
                reduction.max.0 = results.get_max().get_max_x();
                reduction.max.1 = results.get_max().get_max_y();

                println!("Sum: {:?}\nAvg: {:?}\nMin: [{:?}, {:?}]\nMax: [{:?}, {:?}]",
                        reduction.sum, reduction.avg, reduction.min.0, reduction.min.1, reduction.max.0, reduction.max.1);

                println!();
            }
        }
        _ => {
            // TODO Change this to not exit
            panic!("Not a possible analysis!")
        }
    }

    /*End of KJ's Code*/
    Ok(())
}

fn help() {
    println!("Usage: cargo run --bin client <analysis_type>");
    println!("Example: cargo run --bin client Reduction");
    process::exit(1);
}
