pub struct Values {
    pub value: u32,
    pub calc: u32,
}

impl Values {
    pub fn new(_value: u32, _calc: u32) -> Values {
        Values {
            value : _value,
            calc : _calc
        }
    }
}

pub fn calc_avg(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }
    let avg = sum / data.len() as u32;

    println!("Avg: {:?}", avg);

    Values::new(avg, 1)
}

pub fn calc_sum(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }

    println!("Sum: {:?}", sum);

    Values::new(sum, 2)
}

pub fn calc_min(data: Vec<u32>) -> Values {
    let mut min = data[0];
    for value in data.iter()
    {
        if value < &min
        {
            min = *value;
        }
    }

    println!("Min: {:?}", min);

    Values::new(min, 3)
}

pub fn calc_max(data: Vec<u32>) -> Values {
    let mut max = data[0];
    for value in data.iter()
    {
        if value > &max
        {
            max = *value;
        }
    }

    println!("Max: {:?}", max);

    Values::new(max, 4)
}