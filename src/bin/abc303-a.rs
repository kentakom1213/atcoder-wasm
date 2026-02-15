#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::get;

fn main() {
    let _N = get!(usize);
    let S = get!(String);
    let T = get!(String);

    let mut out = stdout();

    for (c, d) in S.chars().zip(T.chars()) {
        match (c, d) {
            ('o', '0') | ('0', 'o') | ('l', '1') | ('1', 'l') => (),
            _ if c == d => (),
            _ => {
                writeln!(out, "No").ok();
                return;
            }
        }
    }

    writeln!(out, "Yes").ok();
}
