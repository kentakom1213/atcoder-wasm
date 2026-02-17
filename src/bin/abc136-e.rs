#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: u64,
        A: [u64; N]
    }

    let S = A.iter().sum::<u64>();

    // S = ΣAi が不変であることから，
    // 条件を満たす p は S の約数に限られる
    let divisors = {
        let mut d = vec![];
        for p in 1.. {
            if p * p > S {
                break;
            }
            if S % p == 0 {
                d.push(p);
                d.push(S / p);
            }
        }
        d.sort_unstable();
        d.dedup();
        d
    };

    debug!(divisors);

    for &p in divisors.iter().rev() {
        if isok(p, K, &A) {
            println!("{p}");
            return;
        }
    }
}

/// A に K 回以下の操作を行うことで，全ての要素を p の倍数にできるか
fn isok(p: u64, K: u64, A: &[u64]) -> bool {
    debug!(p, K, A);

    let rems = {
        let mut r: Vec<_> = A.iter().map(|a| a % p).collect();
        r.sort_unstable();
        r
    };

    // 下げる個数
    let R = rems.iter().sum::<u64>();
    let n = A.len() - (R / p) as usize;

    let cost = rems.iter().take(n).sum::<u64>();

    cost <= K
}
