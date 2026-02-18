#![allow(non_snake_case)]

use cp_library_rs::{debug, utils::boolutil::BoolUtil};
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        println!("{}", solve().yesno());
    }
}

fn solve() -> bool {
    input! {
        N: usize,
        mut A: [i64; N]
    }

    if N == 2 {
        return true;
    }

    // 絶対値が全て同じ場合
    if A.iter().all(|v| v.abs() == A[0].abs()) {
        // 個数をカウント
        let plus = A.iter().filter(|v| **v > 0).count();

        return plus == 0 || plus == N || plus.abs_diff(N - plus) <= 1;
    }

    // 絶対値でソート
    A.sort_unstable_by_key(|v| v.abs());

    debug!(A);

    A[1..]
        .iter()
        .tuple_windows()
        // a/b == A[0]/A[1]
        .all(|(&a, &b)| A[1] * a == A[0] * b)
}
