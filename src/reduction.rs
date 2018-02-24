
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

pub fn calc_sum(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }

    Values::new(sum, 1, Point::new(0, 0))
}

pub fn calc_avg(data: Vec<u32>) -> Values {
    let mut sum = 0;
    for value in data.iter()
    {
        sum += *value;
    }
    let avg = sum / data.len() as u32;

    Values::new(avg, 2, Point::new(0, 0))
}

pub fn calc_min(mz: Vec<i32>, tic: Vec<u32>) -> Values {
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

pub fn calc_max(mz: Vec<i32>, tic: Vec<u32>) -> Values {
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
