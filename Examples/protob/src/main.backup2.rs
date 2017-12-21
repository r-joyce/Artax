extern crate rustc_serialize;
extern crate protobuf;
extern crate snap;
extern crate csv;

// mod protos;
// use protos::message;
// use protobuf::Message;

use csv::{Reader};
use std::error::Error;
use std::process;
use std::env;

#[derive(RustcDecodable, RustcEncodable)]
struct Record {
    tic: f32,
    time: u32,
}

fn encode_data(data: &mut Vec<Message>) -> Result<(), Box<Error>> {
    // Check the content
    for i in data {
        println!("tic: {}, time: {}", i.tic, i.time);
    }

    // Encode data
    let raw = data.write_to_bytes();
    Ok(())
}

fn get_input(file: &String, data: &mut Vec<Record>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_file(file).unwrap().has_headers(false);
    for row in reader.decode() {
        let record: Record = row.unwrap();
        data.push(record);
    }
    Ok(())
}

fn help() {
    println!("[!] Error: Expecting a csv file argument");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file = &args[1];
            println!("[+] Opening {}", file);
            let mut data: Vec<Record> = Vec::new();

            // parse input csv
            if let Err(err) = get_input(file, &mut data) {
                println!("[!] Error processing file: {}", err);
                process::exit(1);
            }

            // encode data
            if let Err(err) = encode_data(&mut data) {
                println!("[!] Error encoding data: {}", err);
                process::exit(1);
            }
        },
        _ => {
            help();
        }
    }
}