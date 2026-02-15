#![allow(non_snake_case)]

use atcoder_wasm::get;

fn main() {
    let (A, B) = get!(u64, u64);

    println!("{}", A.div_ceil(B));
}
