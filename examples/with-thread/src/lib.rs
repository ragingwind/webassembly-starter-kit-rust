// code from https://web.dev/webassembly-threads/
use wasm_bindgen::prelude::*;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
pub fn sum_of_squares(numbers: &[i32]) -> i32 {
  numbers
  .par_iter()
  .map(|x| x * x)
  .sum()
}