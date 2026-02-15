#![allow(non_snake_case)]

use std::io::Write;

use atcoder_wasm::get;

fn main() {
    let _N = get!(usize);
    let T = get!(String);
    let A = get!(String);

    let mut out = std::io::stdout();

    for (c, d) in T.chars().zip(A.chars()) {
        if c == 'o' && d == 'o' {
            writeln!(out, "Yes").ok();
            return;
        }
    }

    writeln!(out, "No").ok();
}
