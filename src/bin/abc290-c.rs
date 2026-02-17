#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N]
    }

    A.sort_unstable();
    A.dedup();

    for (i, a) in (0..K).zip(A.into_iter().chain(std::iter::repeat(usize::MAX))) {
        if i < a {
            println!("{i}");
            return;
        }
    }

    println!("{K}");
}
