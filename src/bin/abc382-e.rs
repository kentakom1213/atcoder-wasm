#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        P: [usize; N],
    }

    let Q = {
        let mut q = vec![0.0; N + 1];
        q[0] = 1.0;
        for (i, &p) in P.iter().enumerate() {
            let p = p as f64 / 100.0;
            for k in (0..=i + 1).rev() {
                let stay0 = q[k] * (1.0 - p);
                let add1 = if k >= 1 { q[k - 1] * p } else { 0.0 };
                q[k] = stay0 + add1;
            }
        }
        q
    };

    debug!(Q);

    // dp[i] := あと i 枚以上集めれば OK である時点で，開封するパックの期待値
    let mut dp = vec![0.0; X + 1];

    for k in 0..=X {
        dp[k] += 1.0;
        for i in 1..k {
            dp[k] += dp[k - i] * Q.get(i).unwrap_or(&0.0);
        }
        dp[k] /= 1.0 - Q[0];
    }

    debug!(dp);

    println!("{:.10}", dp[X]);
}
