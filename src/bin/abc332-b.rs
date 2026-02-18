#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        K: usize,
        G: usize,
        M: usize
    }

    let mut glass = 0;
    let mut mug = 0;

    for _ in 0..K {
        if glass == G {
            glass = 0;
        } else if mug == 0 {
            mug = M;
        } else {
            let x = mug.min(G - glass);
            mug -= x;
            glass += x;
        }
    }

    println!("{glass} {mug}");
}
