#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        P: [usize; N],
    }

    // dp[i] := あと i 枚集めれば OK である時点で，開封するパックの期待値
    let mut dp = vec![0.0; X];

    
}
