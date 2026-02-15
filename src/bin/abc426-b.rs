#![allow(non_snake_case)]

use std::io::Write;

use atcoder_wasm::input;

fn main() {
    let S = input!(String);

    let mut cnt = [0; 26];

    for c in S.chars() {
        cnt[c as usize - 'a' as usize] += 1;
    }

    let mut out = std::io::stdout();

    for (i, c) in ('a'..='z').enumerate() {
        if cnt[i] == 1 {
            writeln!(out, "{c}").ok();
        }
    }
}
