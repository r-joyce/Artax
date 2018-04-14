extern crate protobuf;
extern crate zmq;
extern crate snap;
extern crate config;

mod protos;
mod reduction;

use zmq::{Context, Error};
use std::process;
use std::collections::HashMap;
use protobuf::Message;
use protos::message;
use reduction::*;

fn main() {
    println!("[+] Initializing artax...");
    println!("[+] Loading config file...");
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    for (key, value) in &settings.try_into::<HashMap<String, String>>().unwrap() {
        println!("     {}: {}", key, value);
    }

    // Artax <--> Rust Acquisition Software (server)
    let context = Context::new();
    let server_addr = "tcp://127.0.0.1:25930";
    let subscriber = context.socket(zmq::SUB).unwrap();

    subscriber
        .connect(server_addr)
        .expect("failed connecting subscriber");

    subscriber
        .set_subscribe("data".as_bytes())
        .expect("failed subscribing");

    // Artax <--> Falkor (client)
    let req_addr = "tcp://127.0.0.1:25933";
    let receiver = context.socket(zmq::REP).unwrap();
    receiver
        .bind(req_addr)
        .expect("failed binding router");

    //let sub_addr = "tcp://127.0.0.1:25931";
    //let publisher = context.socket(zmq::PUB).unwrap();
    //publisher
    //    .bind(sub_addr)
    //    .expect("failed binding publisher");

    println!("[+] Begin broker...");

    if let Err(err) = run_broker(receiver, subscriber) {
        println!("[!] Error running Artax: {}", err);
        process::exit(1);
    }
}

// Broker: subscribes to Server, receives requests from Client, publishes results to Client
// Receive data from server, perform client-requested analysis, and publish results back to client
fn run_broker(receiver: zmq::Socket, subscriber: zmq::Socket) -> Result<(), Box<Error>> {

    loop {
        // Receive topic string and data
        let topic = subscriber.recv_string(0).unwrap().unwrap();
        let new_data: Vec<u8> = subscriber.recv_bytes(0).unwrap();
        println!("Message received! Topic {:?}", topic);

        // Decompress data
        let mut reader = snap::Decoder::new();
        let uncompressed_data = reader.decompress_vec(&new_data).unwrap();
        let mut new_message = message::Message::new();
        new_message.merge_from_bytes(&uncompressed_data).unwrap();

        // Poll for client request
        let mut item = [
            receiver.as_poll_item(zmq::POLLIN)
        ];
        zmq::poll(&mut item, -1);

        if item[0].is_readable() {
            //receiver.recv_bytes(0).unwrap();
            //receiver.recv_bytes(0).unwrap();
            let job = receiver.recv_string(0).unwrap().unwrap();

            // Determine analysis
            match job.as_ref() {
                "reduction" => {
                    let mut results = reduction_worker(&mut new_message);

                    // Compress results
                    let mut writer = snap::Encoder::new();
                    let recompressed_data = writer.compress_vec(&results.write_to_bytes().unwrap()).unwrap();

                    // Send results back to the client
                    receiver.send(&recompressed_data, 0).unwrap();
                    println!("Message sent to client!");
                    println!();
                }
                _ => {
                    println!("Analysis does not exist!");
                    process::exit(1);
                }
            }
        }
    }
}

fn reduction_worker(new_message: &mut message::Message) -> message::ReductionMessage {

    // Perform reduction
    let results = reduction(new_message);

    // Package results message to send back to Falkor
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

    new_data
}

fn help() {
    println!("Usage: cargo run --bin artax");
    process::exit(1);
}
