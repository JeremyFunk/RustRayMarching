pub fn min_f64(a: f64, b: f64) -> f64{
    if(a < b){
        return a
    }
    b
}
pub fn max_f64(a: f64, b: f64) -> f64{
    if(a > b){
        return a
    }
    b
}

pub fn vec_f_max(a: [f64;3], val: f64) -> [f64;3]{
    return [max_f64(a[0], val), max_f64(a[1], val), max_f64(a[2], val)]   
}

pub fn vec_max(a: [f64;3], b: [f64;3]) -> [f64;3]{
    return [max_f64(a[0], b[0]), max_f64(a[1], b[1]), max_f64(a[2], b[2])]   
}