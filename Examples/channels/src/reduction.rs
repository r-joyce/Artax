pub fn calc_avg(data: Vec<u64>) -> u64 {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += value;
    }
    let avg = sum / data.len() as u64;
    avg
}

pub fn calc_sum(data: Vec<u64>) -> u64 {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += value;
    }
    sum
}

pub fn calc_min(data: Vec<u64>) -> u64 {
    let mut min = data[0];
    for value in data.iter()
    {
        if value < &min
        {
            min = *value;
        }
    }
    min
}

pub fn calc_max(data: Vec<u64>) -> u64 {
    let mut max = data[0];
    for value in data.iter()
    {
        if value > &max
        {
            max = *value;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avg() {
        assert_eq!(12, calc_avg(vec![8, 13, 7, 9, 23]));
    }
    #[test]
    fn sum() {
        assert_eq!(60, calc_sum(vec![8, 13, 7, 9, 23]));
    }
    #[test]
    fn min() {
        assert_eq!(7, calc_min(vec![13, 84, 7, 34, 57, 9, 90]));
    }
    #[test]
    fn max() {
        assert_eq!(90, calc_max(vec![13, 84, 7, 34, 57, 9, 90]));
    }
}
