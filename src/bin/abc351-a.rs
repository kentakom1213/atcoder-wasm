#![allow(non_snake_case)]

use std::io::Write;

use atcoder_wasm::get;

fn main() {
    let A = get!(usize;;);
    let B = get!(usize;;);

    let ans = A.iter().sum::<usize>() - B.iter().sum::<usize>() + 1;

    let mut out = std::io::stdout();
    writeln!(out, "{ans}").ok();
}
