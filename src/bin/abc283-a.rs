#![allow(non_snake_case)]

use atcoder_wasm::get;

fn main() {
    let (A, B) = get!(u64, u32);

    println!("{}", A.pow(B));
}
