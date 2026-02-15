#![allow(non_snake_case)]

use atcoder_wasm::get;
use std::io::{Write, stdout};

fn main() {
    let N = get!(usize);
    let S = get!(String);

    let mut T = 0;
    let mut A = 0;
    let mut ans: Option<&'static str> = None;

    for c in S.chars() {
        if c == 'T' {
            T += 1;
            if T * 2 > N || T * 2 == N && ans.is_none() {
                ans = Some("T");
            }
        } else {
            A += 1;
            if A * 2 > N || A * 2 == N && ans.is_none() {
                ans = Some("A");
            }
        }
    }

    let mut out = stdout();

    writeln!(out, "{}", ans.unwrap()).ok();
}
