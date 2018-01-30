extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;
extern crate csv;

mod protos;
use protos::message;

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
        time.push(record[0].parse().ok().unwrap()); // Shits broke yo
        tic.push(record[1].parse().ok().unwrap());
    }
    Ok(())
}

fn build_object(data: &mut message::Message, time: Vec<u64>, tic: Vec<u32>) -> Result<(), Box<Error>> {
    data.set_time_stamps(time);
    data.set_tic(tic);
    Ok(())
}

fn compress_data(data: message::Message, comp_data: &mut Vec<u8>) -> Result<(), Box<Error>> {
    let mut writer = snap::Encoder::new();
    *comp_data = writer.compress_vec(&data.write_to_bytes().unwrap()).unwrap();
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
            let num = &args[2].parse::<u32>().unwrap();
            let mut data = message::Message::new();
            let mut tic: Vec<u32> = Vec::new();
            let mut time: Vec<u64> = Vec::new();
            let mut compressed_data: Vec<u8> = Vec::new();

            println!("[+] Opening {}", file);

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
            if let Err(err) = compress_data(data, &mut compressed_data) {
                println!("[!] Error compressing the data: {}", err);
                process::exit(1);
            }

            let iter = 0..*num;
            for i in iter {
                // Pass some stuff to the broker here
                println!("[+] {}", i);
            }

            println!("[+] Done");
        },
        _ => {
            help();
        }
    }
}