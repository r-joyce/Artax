use std::env;

// trait Process {
//     fn Process(&self);
// }

// struct Worker();

// impl Process for Worker {
//     fn Process(&self) {
//         println!("Processing...");
//     }
// }

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("Worker: {:?}", args);

    // let w1 = Worker();
    // let w2 = Worker();

    // let workers: Vec<&Worker> = vec![&w1, &w2];
    // for w in &workers {
    //     w.Process();
    // }
}