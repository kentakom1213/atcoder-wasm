#![allow(non_snake_case)]

use std::io::Write;

use atcoder_wasm::input;

fn main() {
    let A = input!(usize;;);
    let B = input!(usize;;);

    let ans = A.iter().sum::<usize>() - B.iter().sum::<usize>() + 1;

    let mut out = std::io::stdout();
    writeln!(out, "{ans}").ok();
}
