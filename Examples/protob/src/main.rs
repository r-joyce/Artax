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

// fn get_input(file: &String, data: &mut message::Message) -> Result<(), Box<Error>> {
fn get_input(file: &String, tic: &mut Vec<u32>, time: &mut Vec<u64>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_file(file)
        .unwrap()
        .has_headers(false);
    // let mut tic: Vec<u32> = Vec::new();
    // let mut time: Vec<u64> = Vec::new();
    
    for row in reader.records() {
        let record = row?;
        tic.push(record[0]);
        // println!("{:?}", record[0]);
    }

    // for row in reader.decode() {
    //     println!("Yeeeeeeeee");
    // }
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
            let mut data = message::Message::new();
            let mut tic: Vec<u32> = Vec::new();
            let mut time: Vec<u64> = Vec::new();

            println!("[+] Opening {}", file);

            // parse input csv
            if let Err(err) = get_input(file, &mut tic, &mut time) {
                println!("[!] Error processing file: {}", err);
                process::exit(1);
            }

            for i in tic {
                println!("{:?}", i);
            }
            // if let Err(err) = get_input(file, &mut data) {
            //     println!("[!] Error processing file: {}", err);
            //     process::exit(1);
            // }
        },
        _ => {
            help();
        }
    }
}