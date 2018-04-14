use std::process::{Command, Stdio};
use std::cell::RefCell;

fn main() {

    // Vector to manage the processes, should most likely be some kind of queue
    let mut procs: Vec<RefCell<std::process::Child>> = Vec::new();

    // Add the first process to run
    let cmd =
        RefCell::new(Command::new("cmd")
        .args(&["/C", "hello.exe"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap());
    procs.push(cmd);

    // Add the second process
    let cmd2 =
        RefCell::new(Command::new("cmd")
        .args(&["/C", "worker.exe abc 123"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap());
    procs.push(cmd2);

    loop {
        // It's streaming here
        // Broker checks for additional commands here

        procs.retain(move |x| x.borrow_mut().try_wait().unwrap() == None);
        // procs[0].borrow_mut().kill().unwrap();
        // println!("{:?}", procs);
    }
}
