#![allow(non_snake_case)]

use cp_library_rs::number_theory::modint::M998;
use num::Zero;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
    }

    // dp[i][a1][a2][a3] := 長さが i で，長さ j の IS の最後尾として考えられる最小値が aj であるような数列がいくつあるか．
    let mut dp = vec![vec![vec![vec![M998::zero(); M + 2]; M + 2]; M + 2]; N + 1];

    dp[0][M + 1][M + 1][M + 1] = M998::new(1);

    for i in 0..N {
        for a1 in 1..=M + 1 {
            for a2 in 1..=M + 1 {
                for a3 in 1..=M + 1 {
                    for x in 1..=M {
                        // 数列の i+1 項目を x とする
                        let p = dp[i][a1][a2][a3];
                        if x <= a1 {
                            dp[i + 1][x][a2][a3] += p;
                        } else if x <= a2 {
                            dp[i + 1][a1][x][a3] += p;
                        } else if x <= a3 {
                            dp[i + 1][a1][a2][x] += p;
                        }
                    }
                }
            }
        }
    }

    let mut ans = M998::zero();

    for a1 in 1..=M {
        for a2 in a1 + 1..=M {
            for a3 in a2 + 1..=M {
                ans += dp[N][a1][a2][a3];
            }
        }
    }

    println!("{ans}");
}
