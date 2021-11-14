#![allow(unused)]

#[cfg(target_arch = "wasm32")]
use std::arch::wasm32::*;

#[target_feature(enable = "simd128")]
unsafe fn uses_simd(v1: [u32; 4], v2: [u32; 4]) -> v128 {
    let vec_a: v128 = ::std::mem::transmute(v1);
    let vec_b: v128 = ::std::mem::transmute(v2);
    i32x4_mul(vec_a, vec_b)
}

fn main() {
    unsafe {
        let v1: [u32; 4] = [1, 2, 3, 4];
        let v2: [u32; 4] = [10, 20, 30, 40];
        println!("{:?}", uses_simd(v1, v2));
    }
}
