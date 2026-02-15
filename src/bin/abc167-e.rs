//           E - Colorful Blocks
// ----------------------------------------
// 問題
// https://atcoder.jp/contests/abc167/tasks/abc167_e
// ----------------------------------------

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

use cp_library_rs::get;
use cp_library_rs::number_theory::{
    comb::Comb,
    modint::{Fp, M998},
};
use num::Zero;

fn main() {
    let (N, M, K) = get!(usize, usize, usize);

    let comb = Comb::<M998>::new(1_000_000);

    let mut ans = M998::zero();

    for k in 0..=K {
        let mut tmp = M998::from(M);
        tmp *= comb.comb(N - 1, k);
        tmp *= M998::from(M - 1).pow((N - 1 - k) as u64);

        ans += tmp;
    }

    println!("{}", ans);
}
