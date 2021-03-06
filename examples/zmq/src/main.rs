extern crate zmq;

fn main() {
    let ctx = zmq::Context::new();
    
    let socket = ctx.socket(zmq::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send_str("hello", 0).unwrap();
}
