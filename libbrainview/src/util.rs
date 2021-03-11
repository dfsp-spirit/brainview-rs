use ndarray::{Array2};


pub fn a22vec<T: Clone>(a2 : Array2<T>) -> Vec<T> {
    let mut ve : Vec<T> = Vec::with_capacity(a2.len());

    for v in a2.iter() {
        ve.push((*v).clone());
    }
    ve
}

pub fn f32tou32(a: Vec<i32>) -> Vec<u32> {
    let mut u : Vec<u32> = Vec::with_capacity(a.len());

    for v in a.iter() {
        u.push(*v as u32);
    }
    u
}

pub fn vec32minmax(a : Vec<f32>) -> Vec<f32> {
        let mut curv_data_sorted = a.to_vec();
        curv_data_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let min: f32 = curv_data_sorted[0];
        let max: f32 = curv_data_sorted[curv_data_sorted.len() - 1];
        vec![min, max]
}


pub fn scale_to_01(data: Vec<f32>) -> Vec<f32> {
    let mut scaled : Vec<f32> = Vec::with_capacity(data.len());
    for v in data.iter() {
        scaled.push(*v / 5.0); // TODO: implement this.
    }
    let mm = vec32minmax(data);
    let dmin = mm[0];
    let dmax = mm[1];
    println!("TOD: implement scale_to_01");
    scaled
}
