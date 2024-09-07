#![allow(dead_code)]
#![allow(clippy::needless_range_loop)]

use std::ops::Mul;

mod ds;
mod problem;
mod sort;
mod utils;

fn main() {
    let multiply_i32 = multiply(5i32);
    println!("5 * 3 = {}", multiply_i32(3i32)); // 5 * 3 = 15
    let multiply_f64 = multiply(2.5f64);
    println!("2.5 * 3.0 = {}", multiply_f64(3.0f64)); // 2.5 * 3.0 = 7.5
}

fn multiply<T>(a: T) -> impl Fn(T) -> T
where
    T: Copy + Mul<Output = T>,
{
    move |b| a * b
}
