#![allow(non_snake_case)]

use cp_library_rs::utils::consts::Infinity;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        S: String
    }

    let mut ng = 1_usize.wrapping_neg();
    let mut ok = usize::infinity();

    while ok.wrapping_sub(ng) > 1 {
        let mid = ng.wrapping_add(ok) / 2;
        if isok(N, M, mid, &S) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{ok}");
}

fn isok(N: usize, M: usize, k: usize, S: &str) -> bool {
    let mut x = M;
    let mut y = k;

    for c in S.chars() {
        match c {
            '0' => {
                x = M;
                y = k;
            }
            '1' => {
                if x >= 1 {
                    x -= 1;
                } else if y >= 1 {
                    y -= 1;
                } else {
                    return false;
                }
            }
            '2' => {
                if y >= 1 {
                    y -= 1;
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    true
}
