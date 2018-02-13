extern crate zmq;
extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;
extern crate csv;

mod protos;

use protos::message;
use zmq::{Context, SNDMORE};
use csv::{Reader};
use std::error::Error;
use std::process;
use std::env;
use protobuf::Message;

fn get_input(file: &String, time: &mut Vec<u64>, tic: &mut Vec<u32>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_file(file)
        .unwrap()
        .has_headers(false);

    for row in reader.records() {
        let record = row?;
        time.push(record[0].parse().ok().unwrap());
        tic.push(record[1].parse().ok().unwrap());
    }
    Ok(())
}

fn build_object(data: &mut message::Message, time: Vec<u64>, tic: Vec<u32>) -> Result<(), Box<Error>> {
    data.set_time_stamps(time);
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

    let mut results = message::Message::new();
    results.merge_from_bytes(&compressed_data).unwrap();

    let values = results.take_tic();

    let mut j = 0;
    // print the results
    for i in values.iter() {
        if j == 0 {
            println!("Avg: {:?} ", i);
        }
        else if j == 1 {
            println!("Sum: {:?} ", i);
        }
        else if j == 2 {
            println!("Min: {:?} ", i);
        }
        else {
            println!("Max: {:?} ", i);
        }
        j = j + 1;
    }

    println!();

    /*End of KJ's Code*/
    Ok(())
}

fn help() {
    println!("[!] Error: Expecting a csv file argument and an integer for looping");
    println!("Usage: cargo run <my_file.csv> <number_of_times_to_loop>");
    println!("Example: cargo run my_data.csv 10");
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
                let mut time: Vec<u64> = Vec::new();
                let mut compressed_data: Vec<u8> = Vec::new();

                // parse input csv
                if let Err(err) = get_input(file, &mut time, &mut tic) {
                    println!("[!] Error processing file: {}", err);
                    process::exit(1);
                }

                // add the gathered data vectors to a single object
                if let Err(err) = build_object(&mut data, time, tic) {
                    println!("[!] Error adding vectors to data object: {}", err);
                    process::exit(1);
                }

                // compress the data object
                if let Err(err) = compress_data(algorithm, data, &mut compressed_data, &mut ctx, addr) {
                    println!("[!] Error compressing the data: {}", err);
                    process::exit(1);
                }
            }

            println!("[+] Done");
        },
        _ => {
            help();
        }
    }
}
