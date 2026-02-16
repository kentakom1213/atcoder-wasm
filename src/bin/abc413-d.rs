#![allow(non_snake_case)]

use std::collections::HashMap;

use cp_library_rs::{
    chmin, debug,
    number_theory::frac::Frac,
    utils::{boolutil::BoolUtil, consts::Infinity},
};
use itertools::Itertools;
use num::Zero;
use proconio::input;

fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}

fn solve() {
    input! {
        N: usize,
        mut A: [i64; N]
    }
    if N == 2 {
        println!("Yes");
        return;
    }

    A.sort_unstable();

    debug!(A);

    // 一番小さい公比を見つける
    let r = {
        let mut x = (i64::infinity(), true);
        let mut r = Frac::new(i64::infinity(), 1);

        for (&a, &b) in A.iter().tuple_windows() {
            let f = Frac::new(a, b);
            if chmin!(x, (f.numer().abs(), f >= Frac::zero())) {
                r = f;
            }
            if chmin!(x, (f.denom().abs(), f >= Frac::zero())) {
                r = f;
            }
        }
        r
    };

    debug!(r);

    // 逆インデックス
    let mut R = HashMap::<i64, Vec<usize>>::new();
    // 0 は追加しない
    for (&a, i) in A[1..].iter().zip(1..) {
        R.entry(a).or_default().push(i);
    }

    // 到達可能性をチェック
    let mut isok = vec![false; N];
    let mut q = vec![0];
    while let Some(i) = q.pop() {
        isok[i] = true;
        if let Some(suc) = (r * A[i]).as_integer() {
            if let Some(idxs) = R.get_mut(&suc) {
                if let Some(i) = idxs.pop() {
                    q.push(i);
                }
            }
        }
        if let Some(pre) = (Frac::new(A[i], 1) / r).as_integer() {
            if let Some(idxs) = R.get_mut(&pre) {
                if let Some(i) = idxs.pop() {
                    q.push(i);
                }
            }
        }
        debug!(q);
    }

    debug!(isok);

    println!("{}", isok.into_iter().all(|x| x).yesno());
}
