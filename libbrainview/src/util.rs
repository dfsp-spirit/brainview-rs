

/// Convert a vector of `i32` values to `u32`.
///
/// There most likely is some better way to do this in
/// Rust which I have not yet discovered. Maybe `map()` somewhere? Please file an issue
/// if you know it and read this. ;)
pub fn f32tou32(data: Vec<i32>) -> Vec<u32> {
    let mut u : Vec<u32> = Vec::with_capacity(data.len());

    for v in data.iter() {
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
/// # Panics
///
/// If the `data` input vector is empty.
///
/// TODO: Handle NaN (and infinite?) values.
pub fn vec32minmax(data : &Vec<f32>) -> [f32; 2] {
    if (*data).is_empty() {
        panic!("Input data must not be empty.");
    }
    
    let mut curv_data_sorted = data.to_vec();
    curv_data_sorted.sort_by(|a, b| a.partial_cmp(b).expect("Cannot partial_cmp() values, maybe NaNs in data?"));
    let min: f32 = curv_data_sorted[0];
    let max: f32 = curv_data_sorted[curv_data_sorted.len() - 1];
    [min, max]
}


/// Scale the data to range `0..1`.
///
/// # Panics
///
/// If the `data` input vector is empty.
pub fn scale_to_01(data: Vec<f32>) -> Vec<f32> {
    if (data).is_empty() {
        panic!("Input data must not be empty.");
    }
    let mut scaled : Vec<f32> = Vec::with_capacity(data.len());
    let mm = vec32minmax(&data);
    let dmin = mm[0];
    let dmax = mm[1];
    for v in data.iter() {
        scaled.push((*v - dmin) / (dmax - dmin));
    }
    scaled
}


