#![allow(non_snake_case)]

use atcoder_wasm::get;

fn main() {
    let N = get!(usize);
    let S = get!(String; N);

    for s in S.iter().rev() {
        println!("{s}");
    }
}
