#![allow(dead_code)]

use protos::message;
use std::thread;
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Sender, Receiver};

const THREADS: usize = 4;

pub struct Point {
    pub x: i32,
    pub y: u32,
}

impl Point {
    pub fn new(_x: i32, _y: u32) -> Point {
        Point {
            x: _x,
            y: _y,
        }
    }
}

pub struct Values {
    pub value: u32,
    pub calc: u32,
    pub point: Point,
}

impl Values {
    pub fn new(_value: u32, _calc: u32, _point: Point) -> Values {
        Values {
            value : _value,
            calc : _calc,
            point: _point,
        }
    }
}

pub struct Reduction {
    pub sum: u32,
    pub avg: u32,
    pub min: (i32, u32),
    pub max: (i32, u32),
}

impl Reduction {
    pub fn new() -> Reduction {
        Reduction {
            sum: 0,
            avg: 0,
            min: (0, 0),
            max: (0, 0),
        }
    }
}

fn calc_sum(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }

    Values::new(sum, 1, Point::new(0, 0))
}

fn calc_avg(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }
    let avg = sum / data.len() as u32;

    Values::new(avg, 2, Point::new(0, 0))
}

fn calc_min(mz: Vec<i32>, tic: Vec<u32>) -> Values {
    let mut min_tic = tic[0];
    let mut min_mz = mz[0];
    for (index, value) in tic.iter().enumerate()
    {
        if value < &min_tic
        {
            min_tic = *value;
            min_mz = mz[index];
        }
    }

    Values::new(0, 3, Point::new(min_mz, min_tic))
}

fn calc_max(mz: Vec<i32>, tic: Vec<u32>) -> Values {
    let mut max_tic = tic[0];
    let mut max_mz = mz[0];
    for (index, value) in tic.iter().enumerate()
    {
        if value > &max_tic
        {
            max_tic = *value;
            max_mz = mz[index];
        }
    }

    Values::new(0, 4, Point::new(max_mz, max_tic))
}

pub fn reduction(data: &mut message::Message) -> Reduction {
    // Create a new vector containing the tic values
    let mz_values = Arc::new(data.take_mz());
    let tic_values = Arc::new(data.take_tic());
    let mut handles = vec![];
    let (tx, rx): (Sender<Values>, Receiver<Values>) = mpsc::channel();

    // Perform calculations using threads, pass data to receiver using channel
    for _thread in 0..THREADS {
        let thread_tx = tx.clone();
        let mz_values_clone = Arc::clone(&mz_values);
        let tic_values_clone = Arc::clone(&tic_values);
        let handle = thread::spawn(move || {
            if _thread == 0 {
                thread_tx.send(calc_sum(tic_values_clone.to_vec())).expect("should be sum");
            }
            else if _thread == 1 {
                thread_tx.send(calc_max(mz_values_clone.to_vec(), tic_values_clone.to_vec())).expect("should be max");
            }
            else if _thread == 2 {
                thread_tx.send(calc_min(mz_values_clone.to_vec(), tic_values_clone.to_vec())).expect("should be min");
            }
            else {
                thread_tx.send(calc_avg(tic_values_clone.to_vec())).expect("should be avg");
            }
        });
        handles.push(handle);
    }

    // Create a results vector and insert values from receiver channel into it
    // Order: [ avg, sum, min, max ]
    let mut results = Reduction::new();

    for _ in 0..THREADS {
        let temp_data = rx.recv().unwrap();
        match temp_data.calc {
            1 => results.sum = temp_data.value,
            2 => results.avg = temp_data.value,
            3 => {
                    results.min.0 = temp_data.point.x;
                    results.min.1 = temp_data.point.y
                 }
            4 => {
                    results.max.0 = temp_data.point.x;
                    results.max.1 = temp_data.point.y
                 }
            _ => println!("Unexpected calculation returned!"),
        }
    }

    println!("Sum: {:?}\nAvg: {:?}\nMin: [{:?}, {:?}]\nMax: [{:?}, {:?}]",
            results.sum, results.avg, results.min.0, results.min.1, results.max.0, results.max.1);

    // Join child threads so main can continue
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Reduction complete!");

    results
}

pub fn reduction_worker(new_message: &mut message::Message) -> message::ReductionMessage {

    // Perform reduction
    let results = reduction(new_message);

    // Package results message to send back to Falkor
    let mut new_data = message::ReductionMessage::new();
    let mut new_min = message::ReductionMessage_Min::new();
    let mut new_max = message::ReductionMessage_Max::new();
    new_data.set_sum(results.sum);
    new_data.set_avg(results.avg);
    new_min.set_min_x(results.min.0);
    new_min.set_min_y(results.min.1);
    new_data.set_min(new_min);
    new_max.set_max_x(results.max.0);
    new_max.set_max_y(results.max.1);
    new_data.set_max(new_max);

    new_data
}
