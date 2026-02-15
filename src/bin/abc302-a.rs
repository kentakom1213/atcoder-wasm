#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let (A, B) = input!(u64, u64);

    println!("{}", A.div_ceil(B));
}
