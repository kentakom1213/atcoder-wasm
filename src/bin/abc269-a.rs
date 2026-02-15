#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::input;

fn main() {
    let (a, b, c, d) = input!(i32, i32, i32, i32);

    let mut out = stdout();

    writeln!(out, "{}", (a + b) * (c - d)).ok();
    writeln!(out, "Takahashi").ok();
}
