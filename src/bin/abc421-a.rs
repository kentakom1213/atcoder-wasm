#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::input;

fn main() {
    let N = input!(usize);
    let SS = input!(String; N);
    let (X, Y) = input!(usize1, String);

    let mut out = stdout();

    if SS[X] == Y {
        writeln!(out, "Yes").ok();
    } else {
        writeln!(out, "No").ok();
    }
}
