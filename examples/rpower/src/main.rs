fn rp(slice: &[usize], i: usize) -> bool {
    if slice[2] > slice[0] &&
       slice[2] > slice[1] &&
       slice[2] > slice[3] &&
       slice[2] > slice[4]
    {
        let peak:f64 = slice[2] as f64;
        let half_peak = peak/2.0;
        // println!("Peak: {:?}, half: {:?}", peak, half_peak);
        if slice[1] == slice[3] {
            // println!("peak: {:?}, {:?}, {:?}", peak, i+3, i+1);
            let rp = peak/(((i+3) - (i+1)) as f64);
            println!("Method 1: RP = {:?}", rp);
        } else {
            let lhs_y = slice[1] as f64;
            let rhs_y = slice[3] as f64;
            let lhs_x = (i+1) as f64;
            let rhs_x = (i+3) as f64;
            let peak_x = (i+2) as f64;

            let lhs_slope = (peak - lhs_y)/(peak_x - lhs_x);
            let rhs_slope = (rhs_y - peak)/(rhs_x - peak_x);
            
            let calc_rhs_x = ((rhs_slope * rhs_x) + peak - rhs_y)/rhs_slope;
            let calc_lhs_x = ((lhs_slope * lhs_x) + peak - lhs_y)/lhs_slope;

            let rp = peak/(calc_rhs_x - calc_lhs_x);
            println!("peak: {}, lhs_x: {:?}, rhs_x: {:?}", peak, calc_lhs_x, calc_rhs_x);
        }
        return true;
    }
    return false;
}

fn main() {
    let data = vec![0, 0, 0, 1, 2, 5, 2, 1, 0, 0, 3, 8, 7, 2, 0];
    let mut peaks: Vec<(usize, usize)> = Vec::new();

    for (i, &item) in data.iter().enumerate() {
        // println!("i: {:?}, item: {:?}", i, item);
        if i <= (data.len() - 5) {
            if rp(&data[i..i+5], i) {
                peaks.push((i+2, data[i+2]));
            }
        }
    }
    println!("{:?}", peaks);
}
