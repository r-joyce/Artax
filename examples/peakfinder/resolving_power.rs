pub struct Peaks {
    pub peak: u32
}

impl Peaks {
    pub fn new(_peak: u32) -> Peaks {
        Peaks {
            peak : _peak
        }
    }
}

pub fn calc_resolving_power(data: Vec<u32>) -> Peaks {
    let mut peak = 0;
    let mut start = 0;
    let mut end = data.len()-1;
    loop {
        if start <= end {
            let mut mid = start + 1;

            if start == end {
                peak = data[start];
                println!("Peak: {:?}", peak);
                break;
            }

            if data[mid] >= data[mid+1] && data[mid] >= data[mid-1] {
            	peak = data[mid];
	            println!("Peak: {:?}", peak);
            	break;
            }
            else if data[mid] < data[mid-1] {
                end = mid;
            }
            else {
                start = mid;
            }
        }
        else {
            break;
        }
    }

    println!("Peak: {:?}", peak);
    Peaks::new(peak)
}