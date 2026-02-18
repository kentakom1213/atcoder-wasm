#![allow(non_snake_case)]

use cp_library_rs::{chmax, debug};
use proconio::input;

fn main() {
    input! {
        N: usize,
        WHB: [(usize, i64, i64); N]
    }

    // let O = 500 * 500 + 10;
    let O = WHB.iter().map(|(w, _, _)| w).sum::<usize>() + 3;
    let MAX = O * 2;

    debug!(O, MAX);

    // dp[i] := (体の重さ - 頭の重さ) = i のときの嬉しさの最大値
    let mut dp = vec![-1; MAX];
    dp[O] = 0;

    debug!(dp);

    for (w, h, b) in WHB {
        let mut ndp = vec![-1; MAX];

        for i in 0..MAX {
            if dp[i] == -1 {
                continue;
            }

            // 体につける
            chmax!(ndp[i + w], dp[i] + b);
            // 頭につける
            chmax!(ndp[i - w], dp[i] + h);
        }

        dp = ndp;

        debug!(dp);
    }

    let ans = dp[O..].iter().max().unwrap();

    println!("{ans}");
}
