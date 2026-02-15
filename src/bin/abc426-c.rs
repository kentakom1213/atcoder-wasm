#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::get;

fn main() {
    let (N, Q) = get!(usize, usize);

    let mut V = vec![1; N + 1];
    let mut versions: Vec<_> = (1..=N).rev().collect();

    let mut out = stdout();

    for _ in 0..Q {
        let (x, y) = get!(usize, usize);
        let mut updated = 0;

        while versions.last().is_some_and(|&v| v <= x) {
            let v = versions.pop().unwrap();

            updated += V[v];
            V[v] = 0;
        }

        V[y] += updated;

        writeln!(out, "{updated}").ok();
    }
}
