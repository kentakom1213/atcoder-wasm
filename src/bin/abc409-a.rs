#![allow(non_snake_case)]

use std::io::Write;

use atcoder_wasm::input;

fn main() {
    let _N = input!(usize);
    let T = input!(String);
    let A = input!(String);

    let mut out = std::io::stdout();

    for (c, d) in T.chars().zip(A.chars()) {
        if c == 'o' && d == 'o' {
            writeln!(out, "Yes").ok();
            return;
        }
    }

    writeln!(out, "No").ok();
}
