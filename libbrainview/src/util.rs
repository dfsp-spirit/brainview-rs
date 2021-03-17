

/// Convert a vector of `i32` values to `u32`.
///
/// There most likely is some better way to do this in
/// Rust which I have not yet discovered. Maybe `map()` somewhere? Please file an issue
/// if you know it and read this. ;)
pub fn f32tou32(a: Vec<i32>) -> Vec<u32> {
    let mut u : Vec<u32> = Vec::with_capacity(a.len());

    for v in a.iter() {
        u.push(*v as u32);
    }
    u
}


/// Determine the minimum and maximum value of an `f32` vector.
///
/// There most likely is some standard way to do this in
/// Rust which I have not yet discovered. Please file an issue
/// if you know it and read this. ;)
///
/// TODO: Handle NaN (and infinite?) values. Handle empty input vector.
pub fn vec32minmax(a : &Vec<f32>) -> [f32; 2] {
    if (*a).len() < 1 {
        panic!("Input vector must not be empty.");
    }
    
    let mut curv_data_sorted = a.to_vec();
    curv_data_sorted.sort_by(|a, b| a.partial_cmp(b).expect("Cannot partial_cmp() values, maybe NaNs in data?"));
    let min: f32 = curv_data_sorted[0];
    let max: f32 = curv_data_sorted[curv_data_sorted.len() - 1];
    [min, max]
}


/// Scale the data to range `0..1`.
pub fn scale_to_01(data: Vec<f32>) -> Vec<f32> {
    let mut scaled : Vec<f32> = Vec::with_capacity(data.len());
    let mm = vec32minmax(&data);
    let dmin = mm[0];
    let dmax = mm[1];
    for v in data.iter() {
        scaled.push((*v - dmin) / (dmax - dmin));
    }
    scaled
}


