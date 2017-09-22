extern crate capnp;
extern crate capnpc;
mod data_capnp { include!("./schema/msg_capnp.rs"); }

use std::env;
use std::fs::File;
use std::io::Read;

fn encode(contents: String) {
    println!("File contains:\n{}\nEncoding...", contents);
}

fn decode(contents: String) {
    println!("Decoding...");
}

fn compile() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/msg.capnp")
        .run().expect("schema compiler command");
}

fn help() {
    println!("usage:
./capn <encode|decode> <file>
    Encodes/Decodes the file passed.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            // compile();
            let cmd = &args[1];
            let file_arg = &args[2];
            let mut file = File::open(file_arg).expect("[!] Error: File not found!");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("[!] Error: Problem reading the file!");

            match &cmd[..] {
                "encode" => encode(contents),
                "decode" => decode(contents),
                _ => {
                    println!("[!] Error: Unrecognized command!");
                    help();
                    return();
                }
            }
        },
        _ => {
            help();
        }
    }
}
