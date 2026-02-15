#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::input;

fn main() {
    let (N, R, C) = input!(usize, i32, i32);
    let S = input!(String);

    let mut smoke = vec![];

    let (mut or, mut oc) = (0, 0);

    for (s, t) in S.as_bytes().into_iter().zip(0..) {
        smoke.push((or, oc, t));
        match s {
            b'N' => or += 1,
            b'S' => or -= 1,
            b'W' => oc += 1,
            b'E' => oc -= 1,
            _ => (),
        }
    }

    smoke.sort_unstable();

    let mut out = stdout();

    let (mut r, mut c) = (R, C);

    for (s, t) in S.as_bytes().into_iter().zip(0..) {
        match s {
            b'N' => r += 1,
            b'S' => r -= 1,
            b'W' => c += 1,
            b'E' => c -= 1,
            _ => (),
        }

        let pos = smoke.partition_point(|v| (v.0, v.1) < (r, c));

        if pos < N && smoke[pos].0 == r && smoke[pos].1 == c && smoke[pos].2 <= t {
            out.write(&[b'1']).ok();
        } else {
            out.write(&[b'0']).ok();
        }
    }
}
