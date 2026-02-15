#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::input;

fn main() {
    let (R, X) = input!(usize, usize);

    let isok = if X == 1 {
        1600 <= R && R <= 2999
    } else {
        1200 <= R && R <= 2399
    };

    let mut out = stdout();

    if isok {
        writeln!(out, "Yes").ok();
    } else {
        writeln!(out, "No").ok();
    }
}
