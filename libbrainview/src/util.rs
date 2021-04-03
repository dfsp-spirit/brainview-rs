
use neuroformats::vec32minmax;

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
    let (dmin, dmax) = vec32minmax(&data, true);
    for v in data.iter() {
        scaled.push((*v - dmin) / (dmax - dmin));
    }
    scaled
}


