#![allow(non_snake_case)]

use cp_library_rs::{debug, debug2D};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [Usize1; N]
    }

    let mut P: Vec<Vec<u64>> = vec![vec![0]; N];

    for (&a, i) in A.iter().zip(0..) {
        P[a].push(i + 1);
    }

    for i in 0..N {
        P[i].push(N as u64 + 1);
    }

    debug2D!(P);

    let mut ans = 0;

    for i in 0..N {
        let mut tmp = (N as u64 + 2) * (N as u64 + 1) / 2;

        for (&l, &r) in P[i].iter().tuple_windows() {
            let w = r - l;
            tmp -= (w + 1) * w / 2;
        }

        debug!(i, tmp);

        ans += tmp;
    }

    println!("{ans}");
}
