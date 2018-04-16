#![feature(getpid)]

extern crate zmq;
extern crate protobuf;
extern crate snap;

mod protos;
mod reduction;

use zmq::{Context, Error, SNDMORE};
use protobuf::Message;
use protos::message;
use reduction::*;
use std::env;
use std::process;
use reduction::*;


pub enum AnalysisType {
    Reduction,
    ResPower,
    NotSet,
}

pub struct WorkerTask {
    pub id: u32,
    pub analysis_type: AnalysisType,
    pub rep_socket: zmq::Socket,
    pub pull_socket: zmq::Socket,
    pub push_socket: zmq::Socket,
    pub context: zmq::Context,
}


impl WorkerTask {
    pub fn new(analysis: AnalysisType) -> WorkerTask {
        let zmq_context = Context::new();
        let zmq_rep = zmq_context.socket(zmq::REP).unwrap();
        let zmq_pull = zmq_context.socket(zmq::PULL).unwrap();
        let zmq_push = zmq_context.socket(zmq::PUSH).unwrap();

        WorkerTask {
            id: process::id(),
            analysis_type: analysis,
            rep_socket: zmq_rep,
            pull_socket: zmq_pull,
            push_socket: zmq_push,
            context: zmq_context,
        }
    }

    // Connect to REQ-REP, PUSH-PULL & PULL-PUSH sockets
    fn connect(&self, args: Vec<String>) {
        // Worker <--> Artax
        // Set REQ-REP
        let req_addr = format!("{}{}", "tcp://127.0.0.1:", &args[2]);
        self.rep_socket
            .connect(&req_addr)
            .expect("failed connecting to Artax REQ socket");

        // Set PUSH-PULL 1
        let pull_addr = format!("{}{}", "tcp://127.0.0.1:", &args[3]);
        self.pull_socket
            .connect(&pull_addr)
            .expect("failed connecting to Artax PUSH socket");

        // Set PUSH-PULL 2
        let push_addr = format!("{}{}", "tcp://127.0.0.1:", &args[4]);
        self.push_socket
            .connect(&push_addr)
            .expect("failed connecting to Artax PULL socket");
    }

    // TODO Disconnect from REQ-REP, PUSH-PULL & PULL-PUSH sockets
    fn disconnect() {}

    // Receive analysis type on REP socket
    // Receive data on PULL socket
    // Return data message
    fn receive(&mut self) -> message::Message {
        // Receive analysis type
        let analysis = self.rep_socket.recv_string(0).unwrap().unwrap();

        // Send acknowledgement
        self.rep_socket.send("ack", 0).unwrap();

        match analysis.as_ref() {
            // Reduction
            "0" => self.analysis_type = AnalysisType::Reduction,
            // Resolving Power
            "1" => self.analysis_type = AnalysisType::ResPower,
            _ => {
                println!("analysis type unknown");
                std::process::exit(1);
            }
        }

        // Receive data
        let data: Vec<u8> = self.pull_socket.recv_bytes(0).unwrap();
        let mut reader = snap::Decoder::new();
        let uncompressed_data = reader.decompress_vec(&data).unwrap();
        let mut new_message = message::Message::new();
        new_message.merge_from_bytes(&uncompressed_data).unwrap();

        new_message
    }

    // Send reduction results on PUSH socket
    fn send_reduction(&self, results: message::ReductionMessage) {
        // Compress results
        let mut writer = snap::Encoder::new();
        let recompressed_data = writer.compress_vec(&results.write_to_bytes().unwrap()).unwrap();

        self.push_socket.send(&recompressed_data, 0).unwrap();
    }

    // TODO Send resolving power results on PUSH socket
    fn send_respower() {}

    // Run worker analysis
    pub fn run(&mut self, args: Vec<String>) {
        // Connect to sockets
        self.connect(args);

        loop {
            // Receive analysis type and data
            let mut new_message = self.receive();

            // Match on analysis type
            match self.analysis_type {
                AnalysisType::Reduction => {
                    // Run reduction
                    let mut results = reduction::reduction_worker(&mut new_message);
                    self.send_reduction(results);
                }
                AnalysisType::ResPower => {
                    // Run resolving power
                }
                _ => {
                    // Handle unknown analysis type
                }
            }
        }
    }
}


fn main() {
    // Collect port numbers from args
    let args: Vec<String> = env::args().collect();

    if args.len() == 5 {
        // Check analysis type
        let mut analysis: AnalysisType = AnalysisType::NotSet;
        match args[1].as_ref() {
            "0" => analysis = AnalysisType::Reduction,
            "1" => analysis = AnalysisType::ResPower,
            _ => {
                println!("Failed matching analysis type");
                process::exit(1);
            }
        }
        // Run worker
        let mut worker: WorkerTask = WorkerTask::new(analysis);
        worker.run(args);
    }
}

fn help() {
    println!("Usage: worker.exe <analysis_type> <req_rep_port> <push_pull_port> <pull_push_port>");
    println!("Example(reduction): worker.exe 0 5555 5560 5565");
    process::exit(1);
}
