#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::get;

fn main() {
    let (A, B) = get!(usize, usize);

    let mut out = stdout();

    if A + B == 0 {
        writeln!(out, "1").ok();
    } else {
        writeln!(out, "0").ok();
    }
}
