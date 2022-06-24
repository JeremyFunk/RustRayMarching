pub fn min_f64(a: f64, b: f64) -> f64{
    if a < b{
        return a
    }
    b
}
pub fn max_f64(a: f64, b: f64) -> f64{
    if a > b{
        return a
    }
    b
}
#[inline(always)]
pub fn mix(a: f64, b: f64, f: f64) -> f64{
    return a * (1.0 - f) + b * f;
}

pub fn round(v: f64) -> f64{
    return v - (v % 1.0);
}
pub fn vec_interpolate(a: [f64;3], b: [f64;3], fac: f64) -> [f64;3]{
    let inv = 1.0 - fac;
    return [
        a[0] * inv + b[0] * fac, a[1] * inv + b[1] * fac, a[2] * inv + b[2] * fac
    ]
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
    
pub fn mat_dir_mul3(matrix: [[f64;3];3], d: [f64;3]) -> [f64;3]{
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


pub fn mat_rotation3(rot: [f64;3]) -> [[f64;3];3]{
    
    let x = rot[0].to_radians();
    let y = rot[1].to_radians();
    let z = rot[2].to_radians();

    let x_cos = x.cos();
    let x_sin = x.sin();
    let mut x_mat = vecmath::mat3_id();
    
    x_mat[1][1] = x_cos;
    x_mat[1][2] = -x_sin;
    x_mat[2][1] = x_sin;
    x_mat[2][2] = x_cos;

    let y_cos = y.cos();
    let y_sin = y.sin();
    let mut y_mat = vecmath::mat3_id();
    
    y_mat[0][0] = y_cos;
    y_mat[0][2] = y_sin;
    y_mat[2][0] = -y_sin;
    y_mat[2][2] = y_cos;

    
    let z_cos = z.cos();
    let z_sin = z.sin();
    let mut z_mat = vecmath::mat3_id();
    
    z_mat[0][0] = z_cos;
    z_mat[0][1] = -z_sin;
    z_mat[1][0] = z_sin;
    z_mat[1][1] = z_cos;

    return vecmath::row_mat3_mul(vecmath::row_mat3_mul(z_mat, y_mat), x_mat)
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

pub fn mat_rot_scale(rot: [f64;3], scale: [f64;3]) -> [[f64;3];3]{
    let mut rotation_matrix = mat_rotation3(rot);
    rotation_matrix[0][0] *= scale[0];
    rotation_matrix[1][1] *= scale[1];
    rotation_matrix[2][2] *= scale[2];
    return rotation_matrix;
}

const PI_OVER_2: f64 = 1.57079632679489661923;
const PI_OVER_4: f64 = 0.78539816339744830961;


pub fn concentric_sample_disk(sample_x: f64, sample_y: f64) -> Sample{
    let offset_x = 2.0 * sample_x - 1.0;
    let offset_y = 2.0 * sample_y - 1.0;

    if offset_x == 0.0 && offset_y == 0.0{
        return (0.0, 0.0)
    }
    let theta: f64;
    let r: f64;
    if offset_x.abs() > offset_y.abs(){
        r = offset_x;
        theta = PI_OVER_4 * (offset_y / offset_x);
    }else{
        r = offset_y;
        theta = PI_OVER_2 - PI_OVER_4 * (offset_x / offset_y);
    }

    (r * theta.cos(), r * theta.sin())
}

#[inline(always)]
pub fn dot_product(a: [f64;3], b: [f64;3]) -> f64{
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}


#[inline(always)]
pub fn reflect(indicent: [f64;3], normal: [f64;3]) -> [f64;3] {
    let f = 2.0 * dot_product(indicent, normal);
    return [indicent[0] - f * normal[0], indicent[1] - f * normal[1], indicent[2] - f * normal[2]];
}































// Uses a for f64 modified form of https://github.com/fschutt/fastblur
use std::cmp::min;

use crate::Sample;

pub fn gaussian_blur(data: &mut Vec<[f64;3]>, width: usize, height: usize, blur_radius: f64)
{
    let boxes = create_box_gauss(blur_radius, 3);
    let mut backbuf = data.clone();

    for box_size in boxes.iter() {
        let radius = ((box_size - 1) / 2) as usize;
        box_blur(&mut backbuf, data, width, height, radius, radius);
    }
}

/// Same as gaussian_blur, but allows using different blur radii for vertical and horizontal passes
pub fn gaussian_blur_asymmetric(data: &mut Vec<[f64;3]>, width: usize, height: usize, blur_radius_horizontal: f64, blur_radius_vertical: f64) {
    let boxes_horz = create_box_gauss(blur_radius_horizontal, 3);
    let boxes_vert = create_box_gauss(blur_radius_vertical, 3);
    let mut backbuf = data.clone();

    for (box_size_horz, box_size_vert) in boxes_horz.iter().zip(boxes_vert.iter()) {
        let radius_horz = ((box_size_horz - 1) / 2) as usize;
        let radius_vert = ((box_size_vert - 1) / 2) as usize;
        box_blur(&mut backbuf, data, width, height, radius_horz, radius_vert);
    }
}

#[inline]
/// If there is no valid size (e.g. radius is negative), returns `vec![1; len]`
/// which would translate to blur radius of 0
fn create_box_gauss(sigma: f64, n: usize)
-> Vec<i32>
{
    if sigma > 0.0 {
        let n_float = n as f64;

        // Ideal averaging filter width
        let w_ideal = (12.0 * sigma * sigma / n_float).sqrt() + 1.0;
        let mut wl: i32 = w_ideal.floor() as i32;

        if wl % 2 == 0 { wl -= 1; };

        let wu = wl + 2;

        let wl_float = wl as f64;
        let m_ideal = (12.0 * sigma * sigma - n_float * wl_float * wl_float - 4.0 * n_float * wl_float - 3.0 * n_float) /
                    (-4.0 * wl_float - 4.0);
        let m: usize = m_ideal.round() as usize;

        let mut sizes = Vec::<i32>::new();

        for i in 0..n {
            if i < m {
                sizes.push(wl);
            } else {
                sizes.push(wu);
            }
        }

        sizes
    } else {
        vec![1; n]
    }
}

/// Same as gaussian_blur, but allows using different blur radii for vertical and horizontal passes
pub fn gaussian_blur_asymmetric_single_channel(data: &mut Vec<f64>, width: usize, height: usize, blur_radius_horizontal: f64, blur_radius_vertical: f64) {
    let boxes_horz = create_box_gauss(blur_radius_horizontal, 3);
    let boxes_vert = create_box_gauss(blur_radius_vertical, 3);
    let mut backbuf = data.clone();

    for (box_size_horz, box_size_vert) in boxes_horz.iter().zip(boxes_vert.iter()) {
        let radius_horz = ((box_size_horz - 1) / 2) as usize;
        let radius_vert = ((box_size_vert - 1) / 2) as usize;
        box_blur_single_channel(&mut backbuf, data, width, height, radius_horz, radius_vert);
    }
}

/// Needs 2x the same image
#[inline]
fn box_blur(backbuf: &mut Vec<[f64;3]>, frontbuf: &mut Vec<[f64;3]>, width: usize, height: usize, blur_radius_horz: usize, blur_radius_vert: usize)
{
    box_blur_horz(backbuf, frontbuf, width, height, blur_radius_horz);
    box_blur_vert(frontbuf, backbuf, width, height, blur_radius_vert);
}

#[inline]
fn box_blur_vert(backbuf: &[[f64;3]], frontbuf: &mut [[f64;3]], width: usize, height: usize, blur_radius: usize)
{
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f64;

    for i in 0..width {

        let col_start = i; //inclusive
        let col_end = i + width * (height - 1); //inclusive
        let mut ti: usize = i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius * width;

        let fv: [f64;3] = backbuf[col_start];
        let lv: [f64;3] = backbuf[col_end];

        let mut val_r: f64 = (blur_radius as f64 + 1.0) * f64::from(fv[0]);
        let mut val_g: f64 = (blur_radius as f64 + 1.0) * f64::from(fv[1]);
        let mut val_b: f64 = (blur_radius as f64 + 1.0) * f64::from(fv[2]);

        // Get the pixel at the specified index, or the first pixel of the column
        // if the index is beyond the top edge of the image
        let get_top = |i: usize| {
            if i < col_start {
                fv
            } else {
                backbuf[i]
            }
        };

        // Get the pixel at the specified index, or the last pixel of the column
        // if the index is beyond the bottom edge of the image
        let get_bottom = |i: usize| {
            if i > col_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, height) {
            let bb = backbuf[ti + j * width];
            val_r += bb[0];
            val_g += bb[1];
            val_b += bb[2];
        }
        if blur_radius > height {
            val_r += (blur_radius - height) as f64 * lv[0];
            val_g += (blur_radius - height) as f64 * lv[1];
            val_b += (blur_radius - height) as f64 * lv[2];
        }

        for _ in 0..min(height, blur_radius + 1) {
            let bb = get_bottom(ri); ri += width;
            val_r += bb[0] - fv[0];
            val_g += bb[1] - fv[1];
            val_b += bb[2] - fv[2];

            frontbuf[ti] = [val_r * iarr,
                            val_g * iarr,
                            val_b * iarr];
            ti += width;
        }

        if height > blur_radius { // otherwise `(height - blur_radius)` will underflow
            for _ in (blur_radius + 1)..(height - blur_radius) {

                let bb1 = backbuf[ri]; ri += width;
                let bb2 = backbuf[li]; li += width;

                val_r += bb1[0] - bb2[0];
                val_g += bb1[1] - bb2[1];
                val_b += bb1[2] - bb2[2];

                frontbuf[ti] = [val_r * iarr,
                                val_g * iarr,
                                val_b * iarr];
                ti += width;
            }

            for _ in 0..min(height - blur_radius - 1, blur_radius) {
                let bb = get_top(li); li += width;

                val_r += lv[0] - bb[0];
                val_g += lv[1] - bb[1];
                val_b += lv[2] - bb[2];

                frontbuf[ti] = [val_r * iarr,
                                val_g * iarr,
                                val_b * iarr];
                ti += width;
            }
        }
    }
}

#[inline]
fn box_blur_horz(backbuf: &[[f64;3]], frontbuf: &mut [[f64;3]], width: usize, height: usize, blur_radius: usize)
{
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f64;

    for i in 0..height {

        let row_start: usize = i * width; // inclusive
        let row_end: usize = (i + 1) * width - 1; // inclusive
        let mut ti: usize = i * width; // VERTICAL: $i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius;

        let fv: [f64;3] = backbuf[row_start];
        let lv: [f64;3] = backbuf[row_end]; // VERTICAL: $backbuf[ti + $width - 1];

        let mut val_r: f64 = (blur_radius as f64 + 1.0) * fv[0];
        let mut val_g: f64 = (blur_radius as f64 + 1.0) * fv[1];
        let mut val_b: f64 = (blur_radius as f64 + 1.0) * fv[2];

        // Get the pixel at the specified index, or the first pixel of the row
        // if the index is beyond the left edge of the image
        let get_left = |i: usize| {
            if i < row_start {
                fv
            } else {
                backbuf[i]
            }
        };

        // Get the pixel at the specified index, or the last pixel of the row
        // if the index is beyond the right edge of the image
        let get_right = |i: usize| {
            if i > row_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, width) {
            let bb = backbuf[ti + j]; // VERTICAL: ti + j * width
            val_r += bb[0];
            val_g += bb[1];
            val_b += bb[2];
        }
        if blur_radius > width {
            val_r += (blur_radius - height) as f64 * lv[0];
            val_g += (blur_radius - height) as f64 * lv[1];
            val_b += (blur_radius - height) as f64 * lv[2];
        }


        // Process the left side where we need pixels from beyond the left edge
        for _ in 0..min(width, blur_radius + 1) {
            let bb = get_right(ri); ri += 1;
            val_r += bb[0] - fv[0];
            val_g += bb[1] - fv[1];
            val_b += bb[2] - fv[2];

            frontbuf[ti] = [val_r * iarr,
                            val_g * iarr,
                            val_b * iarr];
            ti += 1; // VERTICAL : ti += width, same with the other areas
        }

        if width > blur_radius { // otherwise `(width - blur_radius)` will underflow
            // Process the middle where we know we won't bump into borders
            // without the extra indirection of get_left/get_right. This is faster.
            for _ in (blur_radius + 1)..(width - blur_radius) {

                let bb1 = backbuf[ri]; ri += 1;
                let bb2 = backbuf[li]; li += 1;

                val_r += bb1[0] - bb2[0];
                val_g += bb1[1] - bb2[1];
                val_b += bb1[2] - bb2[2];

                frontbuf[ti] = [val_r * iarr,
                                val_g * iarr,
                                val_b * iarr];
                ti += 1;
            }

            // Process the right side where we need pixels from beyond the right edge
            for _ in 0..min(width - blur_radius - 1, blur_radius) {
                let bb = get_left(li); li += 1;

                val_r += lv[0] - bb[0];
                val_g += lv[1] - bb[1];
                val_b += lv[2] - bb[2];

                frontbuf[ti] = [val_r * iarr,
                                val_g * iarr,
                                val_b * iarr];
                ti += 1;
            }
        }
    }
}

#[inline]
fn box_blur_single_channel(backbuf: &mut [f64], frontbuf: &mut [f64], width: usize, height: usize, blur_radius_horz: usize, blur_radius_vert: usize)
{
    box_blur_horz_single_channel(backbuf, frontbuf, width, height, blur_radius_horz);
    box_blur_vert_single_channel(frontbuf, backbuf, width, height, blur_radius_vert);
}

#[inline]
fn box_blur_vert_single_channel(backbuf: &[f64], frontbuf: &mut [f64], width: usize, height: usize, blur_radius: usize)
{
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f64;

    for i in 0..width {

        let col_start = i; //inclusive
        let col_end = i + width * (height - 1); //inclusive
        let mut ti: usize = i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius * width;

        let fv: f64 = backbuf[col_start];
        let lv: f64 = backbuf[col_end];

        let mut val_r: f64 = (blur_radius as f64 + 1.0) * fv;

        // Get the pixel at the specified index, or the first pixel of the column
        // if the index is beyond the top edge of the image
        let get_top = |i: usize| {
            if i < col_start {
                fv
            } else {
                backbuf[i]
            }
        };

        // Get the pixel at the specified index, or the last pixel of the column
        // if the index is beyond the bottom edge of the image
        let get_bottom = |i: usize| {
            if i > col_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, height) {
            let bb = backbuf[ti + j * width];
            val_r += bb;
        }
        if blur_radius > height {
            val_r += (blur_radius - height) as f64 * lv;
        }

        for _ in 0..min(height, blur_radius + 1) {
            let bb = get_bottom(ri); ri += width;
            val_r += bb - fv;

            frontbuf[ti] = val_r * iarr;
            ti += width;
        }

        if height > blur_radius { // otherwise `(height - blur_radius)` will underflow
            for _ in (blur_radius + 1)..(height - blur_radius) {

                let bb1 = backbuf[ri]; ri += width;
                let bb2 = backbuf[li]; li += width;

                val_r += bb1 - bb2;

                frontbuf[ti] = val_r * iarr;
                ti += width;
            }

            for _ in 0..min(height - blur_radius - 1, blur_radius) {
                let bb = get_top(li); li += width;

                val_r += lv - bb;

                frontbuf[ti] = val_r * iarr;
                ti += width;
            }
        }
    }
}

#[inline]
fn box_blur_horz_single_channel(backbuf: &[f64], frontbuf: &mut [f64], width: usize, height: usize, blur_radius: usize)
{
    if blur_radius == 0 {
        frontbuf.copy_from_slice(backbuf);
        return;
    }

    let iarr = 1.0 / (blur_radius + blur_radius + 1) as f64;

    for i in 0..height {

        let row_start: usize = i * width; // inclusive
        let row_end: usize = (i + 1) * width - 1; // inclusive
        let mut ti: usize = i * width; // VERTICAL: $i;
        let mut li: usize = ti;
        let mut ri: usize = ti + blur_radius;

        let fv: f64 = backbuf[row_start];
        let lv: f64 = backbuf[row_end]; // VERTICAL: $backbuf[ti + $width - 1];

        let mut val_r: f64 = (blur_radius as f64 + 1.0) * fv;

        // Get the pixel at the specified index, or the first pixel of the row
        // if the index is beyond the left edge of the image
        let get_left = |i: usize| {
            if i < row_start {
                fv
            } else {
                backbuf[i]
            }
        };

        // Get the pixel at the specified index, or the last pixel of the row
        // if the index is beyond the right edge of the image
        let get_right = |i: usize| {
            if i > row_end {
                lv
            } else {
                backbuf[i]
            }
        };

        for j in 0..min(blur_radius, width) {
            let bb = backbuf[ti + j]; // VERTICAL: ti + j * width
            val_r += bb;
        }

        if blur_radius > width {
            val_r += (blur_radius - height) as f64 * lv;
        }


        // Process the left side where we need pixels from beyond the left edge
        for _ in 0..min(width, blur_radius + 1) {
            let bb = get_right(ri); ri += 1;
            val_r += bb - fv;

            frontbuf[ti] = val_r as f64 * iarr;
            ti += 1; // VERTICAL : ti += width, same with the other areas
        }

        if width > blur_radius { // otherwise `(width - blur_radius)` will underflow
            // Process the middle where we know we won't bump into borders
            // without the extra indirection of get_left/get_right. This is faster.
            for _ in (blur_radius + 1)..(width - blur_radius) {

                let bb1 = backbuf[ri]; ri += 1;
                let bb2 = backbuf[li]; li += 1;

                val_r += bb1 - bb2;

                frontbuf[ti] = val_r * iarr;
                ti += 1;
            }

            // Process the right side where we need pixels from beyond the right edge
            for _ in 0..min(width - blur_radius - 1, blur_radius) {
                let bb = get_left(li); li += 1;

                val_r += lv - bb;

                frontbuf[ti] = val_r * iarr;
                ti += 1;
            }
        }
    }
}
