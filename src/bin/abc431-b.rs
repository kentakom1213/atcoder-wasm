#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        X: usize,
        N: usize,
        W: [usize; N],
        Q: usize
    }

    let mut state = vec![false; N];
    let mut sum = X;

    for _ in 0..Q {
        input! {
            p: Usize1
        }

        if state[p] {
            sum -= W[p];
        } else {
            sum += W[p];
        }
        state[p] ^= true;

        println!("{sum}");
    }
}
