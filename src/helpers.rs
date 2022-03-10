use std::{rc::Rc, cell::RefCell};

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

pub fn take_vector(vec: [f64!();3]) -> [f64;3]{
    return [
        vec[0].take(),
        vec[1].take(),
        vec[2].take(),
    ]
}

pub fn round(v: f64) -> f64{
    return v - (v % 1.0);
}

pub fn vec_f_max(a: [f64;3], val: f64) -> [f64;3]{
    return [max_f64(a[0], val), max_f64(a[1], val), max_f64(a[2], val)]   
}

pub fn vec_max(a: [f64;3], b: [f64;3]) -> [f64;3]{
    return [max_f64(a[0], b[0]), max_f64(a[1], b[1]), max_f64(a[2], b[2])]   
}

pub fn vec_clamp(v: [f64;3], min: f64, max: f64) -> [f64;3]{
    return [v[0].clamp(min,max), v[1].clamp(min,max), v[2].clamp(min,max)]
}

pub fn mat_vec_mul(matrix: [[f64;4];4], d: [f64;3]) -> [f64;3]{
    let x = d[0] * matrix[0][0] + d[1] * matrix[1][0] + d[2] * matrix[2][0] + matrix[3][0];
    let y = d[0] * matrix[0][1] + d[1] * matrix[1][1] + d[2] * matrix[2][1] + matrix[3][1];
    let z = d[0] * matrix[0][2] + d[1] * matrix[1][2] + d[2] * matrix[2][2] + matrix[3][2];
    let w = 1.0 / (d[0] * matrix[0][3] + d[1] * matrix[1][3] + d[2] * matrix[2][3] + matrix[3][3]);
    return [x * w, y * w, z * w]
}
    
pub fn mat_dir_mul(matrix: [[f64;4];4], d: [f64;3]) -> [f64;3]{
    let x = d[0] * matrix[0][0] + d[1] * matrix[1][0] + d[2] * matrix[2][0];
    let y = d[0] * matrix[0][1] + d[1] * matrix[1][1] + d[2] * matrix[2][1];
    let z = d[0] * matrix[0][2] + d[1] * matrix[1][2] + d[2] * matrix[2][2];
    return [x, y, z]
}

pub fn mat_rotation(rot: [f64;3]) -> [[f64;4];4]{
    
    let x = rot[0].to_radians();
    let y = rot[1].to_radians();
    let z = rot[2].to_radians();

    let x_cos = x.cos();
    let x_sin = x.sin();
    let mut x_mat = vecmath::mat4_id();
    
    x_mat[1][1] = x_cos;
    x_mat[1][2] = -x_sin;
    x_mat[2][1] = x_sin;
    x_mat[2][2] = x_cos;

    let y_cos = y.cos();
    let y_sin = y.sin();
    let mut y_mat = vecmath::mat4_id();
    
    y_mat[0][0] = y_cos;
    y_mat[0][2] = y_sin;
    y_mat[2][0] = -y_sin;
    y_mat[2][2] = y_cos;

    
    let z_cos = z.cos();
    let z_sin = z.sin();
    let mut z_mat = vecmath::mat4_id();
    
    z_mat[0][0] = z_cos;
    z_mat[0][1] = -z_sin;
    z_mat[1][0] = z_sin;
    z_mat[1][1] = z_cos;

    return vecmath::row_mat4_mul(vecmath::row_mat4_mul(x_mat, y_mat), z_mat)
}

pub fn mat_scale(scale: [f64;3]) -> [[f64;4];4]{
    let mut m = vecmath::mat4_id();
    m[0][0] = scale[0];
    m[1][1] = scale[1];
    m[2][2] = scale[2];
    return m
}

pub fn mat_transformation(pos: [f64;3], rot: [f64;3], scale: [f64;3]) -> [[f64;4];4]{
    
    let rotation_matrix = mat_rotation(rot);
    let mut transformation_matrix = vecmath::row_mat4_mul(rotation_matrix, mat_scale(scale));
    transformation_matrix[3][0] = pos[0];
    transformation_matrix[3][1] = pos[1];
    transformation_matrix[3][2] = pos[2];
    return transformation_matrix
}

pub fn mat_pos_rot(pos: [f64;3], rot: [f64;3]) -> [[f64;4];4]{
    
    let mut rotation_matrix = mat_rotation(rot);
    rotation_matrix[3][0] = pos[0];
    rotation_matrix[3][1] = pos[1];
    rotation_matrix[3][2] = pos[2];
    return rotation_matrix
}