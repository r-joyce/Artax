use std::process::{Command, Stdio};

fn main() {

    let mut procs: Vec<std::process::Child>;
    let mut cmd =
        Command::new("cmd")
        .args(&["/C", "cargo run --bin hello"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let mut cmd2 =
        Command::new("cmd")
        .args(&["/C", "cargo run --bin worker abc 123"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    procs.append(cmd);
    // procs.append(cmd2);

    loop {
        // It's streaming here

        let status = cmd.try_wait();
        let status2 = cmd2.try_wait();
        println!("Exited with status {:?}", status);
        println!("Exited with status {:?}", status2);
    }    
}