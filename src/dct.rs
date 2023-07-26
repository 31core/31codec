use crate::mat::Matrix;
use std::f64::consts::PI;

pub const SIZE: usize = 8;

pub fn q_mat() -> Matrix<f64> {
    Matrix::from(
        &[
            16., 11., 10., 16., 24., 40., 51., 61., // row 1
            12., 12., 14., 19., 26., 58., 60., 55., // row 2
            14., 13., 16., 24., 40., 57., 69., 56., // row 3
            14., 17., 22., 29., 51., 87., 80., 62., // row 4
            18., 22., 37., 56., 68., 109., 103., 77., // row 5
            24., 35., 55., 64., 81., 104., 113., 92., // row 6
            49., 64., 78., 87., 103., 121., 120., 101., // row 7
            72., 92., 95., 98., 112., 100., 103., 99., // row 8
        ],
        SIZE,
    )
}

fn c(i: f64) -> f64 {
    if i == 0. {
        (1. / SIZE as f64).sqrt()
    } else {
        (2. / SIZE as f64).sqrt()
    }
}

fn dct_point(u: f64, v: f64, tail: &Matrix<f64>) -> f64 {
    let mut sum = 0.;
    let n = tail.len() as f64;
    for i in 0..tail.len() {
        for j in 0..tail.len() {
            sum += tail.get(i, j)
                * ((i as f64 + 0.5) * PI / n * u).cos()
                * ((j as f64 + 0.5) * PI / n * v).cos();
        }
    }
    c(u) * c(v) * sum
}

fn idct_point(u: f64, v: f64, tail: &Matrix<f64>) -> f64 {
    let mut sum = 0.;
    let n = tail.len() as f64;
    for i in 0..tail.len() {
        for j in 0..tail.len() {
            sum += c(i as f64)
                * c(j as f64)
                * tail.get(i, j)
                * ((u + 0.5) * PI / n * i as f64).cos()
                * ((v + 0.5) * PI / n * j as f64).cos();
        }
    }
    sum
}

pub fn dct(tail: &mut Matrix<f64>) {
    let src = tail.clone();
    for u in 0..SIZE {
        for v in 0..SIZE {
            tail.set(u, v, dct_point(u as f64, v as f64, &src));
        }
    }
}

pub fn idct(tail: &mut Matrix<f64>) {
    let src = tail.clone();
    for u in 0..tail.len() {
        for v in 0..SIZE {
            tail.set(u, v, idct_point(u as f64, v as f64, &src));
        }
    }
}

pub fn quantize(tail: &mut Matrix<f64>) {
    let q_mat = q_mat();
    for i in 0..tail.len() {
        for j in 0..tail.len() {
            let e = tail.get(i, j) / q_mat.get(i, j);
            tail.set(i, j, e);
        }
    }
}

pub fn iquantize(tail: &mut Matrix<f64>) {
    let q_mat = q_mat();
    for i in 0..SIZE {
        for j in 0..SIZE {
            let e = tail.get(i, j) * q_mat.get(i, j);
            tail.set(i, j, e);
        }
    }
}

fn f64_to_i8(tail: &Matrix<f64>) -> Matrix<i8> {
    tail.convert(|e| {
        if e - (e as i8 as f64) < 0.5 {
            e as i8
        } else {
            e as i8 + 1
        }
    })
}

fn i8_to_f64(tail: &Matrix<i8>) -> Matrix<f64> {
    tail.convert(|e| e as f64)
}

fn u8_to_i8(tail: &Matrix<u8>) -> Matrix<i8> {
    tail.convert(|e| (e as isize - 128) as i8)
}

fn i8_to_u8(tail: &Matrix<i8>) -> Matrix<u8> {
    tail.convert(|e| (e as isize + 128) as u8)
}

pub fn encode(tail: &Matrix<u8>) -> Matrix<u8> {
    let mut tail = i8_to_f64(&u8_to_i8(tail));
    dct(&mut tail);
    quantize(&mut tail);
    i8_to_u8(&f64_to_i8(&tail))
}

pub fn decode(tail: &Matrix<u8>) -> Matrix<u8> {
    let mut tail = i8_to_f64(&u8_to_i8(tail));
    iquantize(&mut tail);
    idct(&mut tail);
    i8_to_u8(&f64_to_i8(&tail))
}
