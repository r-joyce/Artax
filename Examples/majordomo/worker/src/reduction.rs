
pub fn calc_avg(data: Vec<u32>) -> u32 {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }
    let avg = sum / data.len() as u32;

    println!("Avg: {:?}", avg);
    avg
}

pub fn calc_sum(data: Vec<u32>) -> u32 {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }

    println!("Sum: {:?}", sum);
    sum
}

pub fn calc_min(data: Vec<u32>) -> u32 {
    let mut min = data[0];
    for value in data.iter()
    {
        if value < &min
        {
            min = *value;
        }
    }

    println!("Min: {:?}", min);
    min
}

pub fn calc_max(data: Vec<u32>) -> u32 {
    let mut max = data[0];
    for value in data.iter()
    {
        if value > &max
        {
            max = *value;
        }
    }

    println!("Max: {:?}", max);
    max
}
