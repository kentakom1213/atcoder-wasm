#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let N = input!(usize);
    let S = input!(String; N);

    for s in S.iter().rev() {
        println!("{s}");
    }
}
