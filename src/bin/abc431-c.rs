#![allow(non_snake_case)]

use cp_library_rs::utils::boolutil::BoolUtil;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        mut H: [u64; N],
        mut B: [u64; M]
    }

    H.sort_unstable();
    B.sort_unstable();

    let mut cnt = 0;

    let mut i = 0;
    let mut j = 0;

    while i < N && j < M {
        if H[i] <= B[j] {
            cnt += 1;
            i += 1;
            j += 1;
        } else {
            j += 1;
        }
    }

    println!("{}", (cnt >= K).yesno());
}
