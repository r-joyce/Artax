pub fn calc_avg(thread_num: usize, data: Vec<u32>) {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }
    let avg = sum / data.len() as u32;

    println!("Thread: {:?} -> Average: {:?}", thread_num, avg);
}

pub fn calc_sum(thread_num: usize, data: Vec<u32>) {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }

    println!("Thread: {:?} -> Sum: {:?}", thread_num, sum);
}

pub fn calc_min(thread_num: usize, data: Vec<u32>) {
    let mut min = data[0];
    for value in data.iter()
    {
        if value < &min
        {
            min = *value;
        }
    }

    println!("Thread: {:?} -> Min: {:?}", thread_num, min);
}

pub fn calc_max(thread_num: usize, data: Vec<u32>) {
    let mut max = data[0];
    for value in data.iter()
    {
        if value > &max
        {
            max = *value;
        }
    }

    println!("Thread: {:?} -> Max: {:?}", thread_num, max);
}
