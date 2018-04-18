#![allow(dead_code)]

extern crate zmq;
extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;

mod protos;

use protobuf::Message;
use protos::message;
use zmq::{Context, SNDMORE};
//use std::process;
use std::thread;
use std::time::Duration;
use std::f64;


fn main() {
    println!("[+] Initializing server...");

    // Set up publisher socket
    let context = Context::new();
    let sub_addr = "tcp://127.0.0.1:25930";
    let publisher = context.socket(zmq::PUB).unwrap();
    publisher
        .bind(sub_addr)
        .expect("failed binding publisher");

    let mut multiply: f64 = 1.0;
    let mut scale: i32 = 1;

    println!("[+] Begin publishing data...");

    // Publish simulated raw data every 16 ms
    loop {
        // Create data to send
        let mz = create_mz(scale as i32);
        let tic = create_tic(multiply);
        let timestamp = create_timestamp(scale as u64);
        multiply += 0.5;
        scale += 1;

        // Create new message and fill it
        let mut new_data = message::Message::new();
        new_data.set_mz(mz);
        new_data.set_tic(tic);
        new_data.set_time_stamps(timestamp);

        // Compress the data
        let mut writer = snap::Encoder::new();
        let compressed_data = writer.compress_vec(&new_data.write_to_bytes().unwrap()).unwrap();

        // Send Topic Envelope
        publisher
            .send("data", SNDMORE)
            .expect("failed sending topic");

        // Send Data
        publisher
            .send(&compressed_data, 0)
            .expect("failed sending data");

        // Wait 16 ms
        thread::sleep(Duration::from_millis(16));
    }
}


/* Create a vector of mz values */
fn create_mz(scale: i32) -> Vec<i32> {
    let mut mz_vec: Vec<i32> = Vec::with_capacity(155000);

    for i in 0..154999 {
        mz_vec.push(i + scale);
    }

    mz_vec
}

/* Create a vector of sine wave points */
fn create_tic(multiply: f64) -> Vec<u32> {
    let mut tic_vec: Vec<u32> = Vec::with_capacity(155000);

    for i in 0..154999 {
        tic_vec.push(((i as f64).sin().abs() * multiply) as u32);
    }

    tic_vec
}

fn create_timestamp(scale: u64) -> Vec<u64> {
    let mut timestamp_vec: Vec<u64> = Vec::with_capacity(155000);

    for i in 0..154999 {
        timestamp_vec.push(i + scale);
    }

    timestamp_vec
}

/*
fn help() {
    println!("Usage: cargo run --bin server");
    process::exit(1);
}
*/
