#![allow(non_snake_case)]

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [u64; N]
    }

    let N = N as u64;

    A.sort();

    // 操作後の最小値を求める
    let sum = A.iter().sum::<u64>();
    let mut x = sum / N;
    let r = sum % N;

    let mut plus = 0;
    let mut minus = 0;

    for (&a, i) in A.iter().zip(0..) {
        if i == N - r {
            x += 1;
        }
        if a < x {
            plus += x - a;
        }
        if a > x {
            minus += a - x;
        }
    }

    debug!(plus, minus);

    assert_eq!(plus, minus);

    println!("{plus}");
}
