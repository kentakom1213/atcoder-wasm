#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::input;

fn main() {
    let _N = input!(usize);
    let S = input!(String);
    let T = input!(String);

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
