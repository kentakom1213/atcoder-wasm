#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let N = input!(usize);
    let SS = input!(String; N);

    let m = SS.iter().map(|s| s.len()).max().unwrap();

    for s in SS {
        let p = m - s.len();
        let pad = ".".repeat(p / 2);
        println!("{pad}{s}{pad}");
    }
}
