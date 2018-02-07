use std::thread;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};

mod reduction;

const THREADS: usize = 4;

fn main() {
    let values = Arc::new(vec![3,6,2,13,76,23,18,51,8,116,27,99]);
    let mut handles = vec![];
    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();

    print!("Vector: [ ");
    for i in values.iter() {
        print!("{:?} ", i);
    }
    println!("]");

    for _thread in 0..THREADS {
        let thread_tx = tx.clone();
        let values_clone = Arc::clone(&values);
        let handle = thread::spawn(move || {
            if _thread == 0 {
                thread_tx.send(reduction::calc_avg(values_clone.to_vec())).expect("should be avg");
            }
            else if _thread == 1 {
                thread_tx.send(reduction::calc_sum(values_clone.to_vec())).expect("should be sum");
            }
            else if _thread == 2 {
                thread_tx.send(reduction::calc_min(values_clone.to_vec())).expect("should be min");
            }
            else {
                thread_tx.send(reduction::calc_max(values_clone.to_vec())).expect("should be max");
            }
        });
        handles.push(handle);
    }

    let mut ids = Vec::with_capacity(THREADS);
    for _ in 0..THREADS {
        ids.push(rx.recv());
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", ids);
 
    println!("Reduction complete!");
}
