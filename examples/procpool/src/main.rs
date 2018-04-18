use std::process::{Command, Child, Stdio};

fn main() {

    let mut procs: Vec<Child> = Vec::new();
    let mut cmd =
        Command::new("cmd")
        .args(&["/C", "hello.exe"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    procs.push(cmd);

    let mut cmd2 =
        Command::new("cmd")
        .args(&["/C", "worker.exe abc 123"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    procs.push(cmd2);

    loop {
        // It's streaming here
        // Broker checks for additional commands here

        for p in &mut procs {
            let status = p.try_wait();

            match status {
                Ok(None) => {
                    println!("[{:?}] Running...", p.id());
                }
                Ok(Some(T)) => {
                    println!("[{:?}] Status: {:?}", p.id(), status);
                    // let index = procs.iter().position(|x| *x == p).unwrap();
                    // procs.remove(index);
                    // let x = procs.remove_item(&p);
                }
                _err => {
                    println!("Hurr durr, there was an errur");
                }
            }
        }
    }
}
