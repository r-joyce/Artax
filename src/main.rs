#![allow(dead_code)]

extern crate protobuf;
extern crate zmq;
extern crate snap;
extern crate config;

mod reduction;
mod protos;

use zmq::{Context, Error, SNDMORE};
use std::process::*;
use std::cell::RefCell;
use std::collections::HashMap;


fn main() {
    println!("[+] Initializing artax...");

    // Load config file port settings
    println!("[+] Loading config file...");
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();
    let x = &settings.try_into::<HashMap<String, String>>().unwrap();

    // Artax <--> Rust Acquisition Software (server)
    let context = Context::new();
    let server_addr = format!("{}:{}", &x.get("host").unwrap(), &x.get("acquisition_sub_port").unwrap());
    let subscriber = context.socket(zmq::SUB).unwrap();

    subscriber
        .connect(&server_addr)
        .expect("failed connecting subscriber");

    subscriber
        .set_subscribe("data".as_bytes())
        .expect("failed subscribing");

    // Artax <--> Falkor (client)
    let req_addr = format!("{}:{}", &x.get("host").unwrap(), &x.get("falkor_reply_port").unwrap());
    let receiver = context.socket(zmq::REP).unwrap();
    receiver
        .bind(&req_addr)
        .expect("failed binding router");

    let sub_addr = format!("{}:{}", &x.get("host").unwrap(), &x.get("falkor_pub_port").unwrap());
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind(&sub_addr)
        .expect("failed binding publisher");

    // Artax <--> Workers
    // Reduction worker
    let red_worker_rep_addr = "tcp://127.0.0.1:5555";
    let red_worker_pull_addr = "tcp://127.0.0.1:5560";
    let red_worker_push_addr = "tcp://127.0.0.1:5565";
    let red_req = context.socket(zmq::REQ).unwrap();
    red_req
        .bind(red_worker_rep_addr)
        .expect("failed binding reduction worker REQ-REP");
    let red_push = context.socket(zmq::PUSH).unwrap();
    red_push
        .bind(red_worker_pull_addr)
        .expect("failed binding reduction worker PUSH-PULL");
    let red_pull = context.socket(zmq::PULL).unwrap();
    red_pull
        .bind(red_worker_push_addr)
        .expect("failed binding reduction worker PULL-PUSH");

    // Resolving Power worker
    let res_worker_rep_addr = "tcp://127.0.0.1:5556";
    let res_worker_pull_addr = "tcp://127.0.0.1:5561";
    let res_worker_push_addr = "tcp://127.0.0.1:5566";
    let res_req = context.socket(zmq::REQ).unwrap();
    res_req
        .bind(res_worker_rep_addr)
        .expect("failed binding reduction worker REQ-REP");
    let res_push = context.socket(zmq::PUSH).unwrap();
    res_push
        .bind(res_worker_pull_addr)
        .expect("failed binding reduction worker PUSH-PULL");
    let res_pull = context.socket(zmq::PULL).unwrap();
    res_pull
        .bind(res_worker_push_addr)
        .expect("failed binding reduction worker PULL-PUSH");


    println!("[+] Begin broker...");


    if let Err(err) =
        run_broker(receiver, subscriber, publisher, red_req, red_push, red_pull, res_req, res_push, res_pull) {
            println!("[!] Error running Artax: {}", err);
            std::process::exit(1);
        }
}

// Broker: subscribes to Server, receives requests from Client, publishes results to Client
// Receive data from server, perform client-requested analysis, and publish results back to client
fn run_broker(
    receiver: zmq::Socket,
    subscriber: zmq::Socket,
    publisher: zmq::Socket,
    red_req: zmq::Socket,
    red_push: zmq::Socket,
    red_pull: zmq::Socket,
    _res_req: zmq::Socket,
    _res_push: zmq::Socket,
    _res_pull: zmq::Socket
) -> Result<(), Box<Error>> {

    // Workers
    let mut worker_procs: Vec<RefCell<Child>> = Vec::new();
    let reduction = RefCell::new(Command::new("cmd").args(&["/C", "worker.exe 0 5555 5560 5565"]).spawn().unwrap());
    let res_power = RefCell::new(Command::new("cmd").args(&["/C", "worker.exe 1 5556 5561 5566"]).spawn().unwrap());
    worker_procs.push(reduction);
    worker_procs.push(res_power);

    loop {
        // Receive topic string and data
        let topic = subscriber.recv_string(0).unwrap().unwrap();
        let new_data: Vec<u8> = subscriber.recv_bytes(0).unwrap();
        println!("Message received! Topic {:?}", topic);

        // Poll for client request
        let mut item = [
            receiver.as_poll_item(zmq::POLLIN)
        ];
        zmq::poll(&mut item, -1).unwrap();

        if item[0].is_readable() {
            let job = receiver.recv_string(0).unwrap().unwrap();
            receiver.send("ack", 0).unwrap();

            // Determine analysis
            match job.as_ref() {
                // Reduction
                "reduction" => {
                    // Run Worker
                    // Send reduction enum to worker, receive acknowledgement
                    red_req.send("0", 0).unwrap();
                    let _ = red_req.recv_string(0).unwrap().unwrap();

                    // Send data to worker
                    red_push.send(&new_data, 0).unwrap();

                    // Receive results from worker
                    let results: Vec<u8> = red_pull.recv_bytes(0).unwrap();

                    // Send results back to the client
                    publisher.send("reduction", SNDMORE).unwrap();
                    publisher.send(&results, 0).unwrap();
                    println!("Message sent to client!");
                    println!();
                }
                // Resolving Power
                //"1" => {

                //}
                _ => {
                    println!("Analysis does not exist!");
                    std::process::exit(1);
                }
            }
        }
        worker_procs.retain(move |x| x.borrow_mut().try_wait().unwrap() == None);
    }

    Ok(())
}

/*
fn help() {
    println!("Usage: cargo run --bin artax");
    std::process::exit(1);
}
*/
