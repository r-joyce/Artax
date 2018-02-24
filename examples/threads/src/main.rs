use std::thread;
use std::sync::Arc;

mod reduction;

const THREADS: usize = 4;

fn main() {
    let values = Arc::new(vec![3,6,2,13,76,23,18,51,8,116,27,99]);
    let mut handles = vec![];

    print!("Vector: [ ");
    for i in values.iter() {
        print!("{:?} ", i);
    }
    println!("]");

    for _thread in 0..THREADS {
        let values_clone = Arc::clone(&values);
        let handle = thread::spawn(move || {
            if _thread == 0 {
                reduction::calc_avg(_thread, values_clone.to_vec());
            }
            else if _thread == 1 {
                reduction::calc_sum(_thread, values_clone.to_vec());
            }
            else if _thread == 2 {
                reduction::calc_min(_thread, values_clone.to_vec());
            }
            else {
                reduction::calc_max(_thread, values_clone.to_vec());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Reduction complete!");
}
