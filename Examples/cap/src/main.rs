extern crate capnp;
extern crate capnpc;
mod capnp_msg { include!("./schema/msg_capnp.rs"); }

use std::env;
use std::fs::File;
use std::io::Read;
use capnp_msg::time_stamp_message as TimeStamp;
use capnp_msg::tic_message as Tic;
use capnp_msg::mz_message as Mz;

fn encode(contents: String) {
    let mut message = ::capnp::message::Builder::new_default();
    let ts = message.init_root::<TimeStamp::Builder>();
    // let tic = message.init_root::<Tic::Builder>();
    // let mz = message.init_root::<Mz::Builder>();
    let line_count = contents.lines().count();
    for i in 0..line_count {
        match i {
            0 => {
                let line = contents.lines().nth(0).unwrap();
                println!("Line 1: {:?}", line);
                let comma_count = line.split(",").count(); // Theres actually -1, but it makes looping easier =)
                println!("Commas: {}", comma_count);
                for j in 0..comma_count {
                    let split = line.split(",").nth(j).unwrap();
                    ts.borrow().set_time_stamp(split);
                    println!(" {}", split);
                }
            },
            1 => {
                let line = contents.lines().nth(1).unwrap();
                println!("Line 2: {:?}", line);
                let comma_count = line.split(",").count();
                println!("Commas: {}", comma_count);
                for j in 0..comma_count {
                    let split = line.split(",").nth(j).unwrap();
                    println!(" {}", split);
                }
            },
            2 => {
                let line = contents.lines().nth(2).unwrap();
                println!("Line 3: {:?}", line);
                let comma_count = line.split(",").count();
                println!("Commas: {}", comma_count);
                for j in 0..comma_count {
                    let split = line.split(",").nth(j).unwrap();
                    println!(" {}", split);
                }
            },
            _ => {
                println!("Unrecognized format");
            }
        }
    }


    // for line in contents.lines() {
    //     let length = line.split(",").count();
    //     for i in 0..length {
    //         let split = line.split(",").nth(i).unwrap();
    //         println!("{} + ", split);
    //     }
        // for s in split {
        //     println!("{} + ", s);
        // }
    // }
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
            // compile(); // Requires having capnp installed on your system
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
