extern crate zmq;
extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;
extern crate csv;

mod protos;
mod reduction;
mod resolving_power;

use protos::message;
use zmq::{Context, SNDMORE};
use csv::{Reader};
use std::error::Error;
use std::process;
use std::env;
use protobuf::Message;
use reduction::*;
use resolving_power::*;

fn get_input(file: &String, mz: &mut Vec<i32>, tic: &mut Vec<u32>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_file(file)
        .unwrap()
        .has_headers(false);

    let mut count = 0;

    for row in reader.records() {
        let record = row?;
        mz.push(count);
        tic.push(record[1].parse().ok().unwrap());
        count += 1;
    }
    Ok(())
}

fn build_object(data: &mut message::Message, mz: Vec<i32>, tic: Vec<u32>) -> Result<(), Box<Error>> {
    data.set_mz(mz);
    data.set_tic(tic);
    Ok(())
}

// Compress and send data
fn compress_data(algorithm: &String, data: message::Message, comp_data: &mut Vec<u8>, ctx: &mut Context, addr: &str) -> Result<(), Box<Error>> {
    let mut writer = snap::Encoder::new();
    *comp_data = writer.compress_vec(&data.write_to_bytes().unwrap()).unwrap();
    let sock = ctx.socket(zmq::REQ)?;
    sock.connect(addr)?;

    // send the data to the broker
    match &algorithm as &str {
        "reduction" => sock.send("Reduction".as_ref(), SNDMORE)?,
        "thrash" => sock.send("Thrash".as_ref(), SNDMORE)?,
        "msms" => sock.send("Msms".as_ref(), SNDMORE)?,
        "resolution" => sock.send("Resolution".as_ref(), SNDMORE)?,
        _ => panic!("Not a possible analysis!"),
    }
    sock.send(&comp_data, 0)?;

    // received the message from the broker
    let received_message: i32 = Default::default();
    let worker_res: Vec<u8> = sock.recv_bytes(received_message)?;

    // decompress the data
    let mut reader = snap::Decoder::new();
    let compressed_data = reader.decompress_vec(&worker_res).unwrap();

    let mut results = message::ReductionMessage::new();
    results.merge_from_bytes(&compressed_data).unwrap();

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

    Ok(())
}

fn help() {
    println!("[!] Error: Expecting a csv file argument and an integer for looping");
    println!("Usage: cargo run --bin client <my_file.csv> <analysis_type>");
    println!("Example: cargo run --bin client my_data.csv Reduction");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            // gather file name and initialize variables
            let file = &args[1];
            let algorithm = &args[2].to_lowercase();

            let mut ctx = Context::new();
		    let addr = "tcp://127.0.0.1:25933";

            println!("[+] Opening {}", file);

            loop {
                let mut data = message::Message::new();
                let mut tic: Vec<u32> = Vec::new();
                let mut mz: Vec<i32> = Vec::new();
                let mut compressed_data: Vec<u8> = Vec::new();

                // parse input csv
                if let Err(err) = get_input(file, &mut mz, &mut tic) {
                    println!("[!] Error processing file: {}", err);
                    process::exit(1);
                }

                // add the gathered data vectors to a single object
                if let Err(err) = build_object(&mut data, mz, tic) {
                    println!("[!] Error adding vectors to data object: {}", err);
                    process::exit(1);
                }

                // compress the data object
                if let Err(err) = compress_data(algorithm, data, &mut compressed_data, &mut ctx, addr) {
                    println!("[!] Error compressing the data: {}", err);
                    process::exit(1);
                }
            }
        },
        _ => {
            help();
        }
    }
}
