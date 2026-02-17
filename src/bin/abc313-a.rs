#![allow(non_snake_case)]

use cp_library_rs::chmax;
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize; N]
    }

    let mut mx = 0;

    for &p in &P[1..] {
        chmax!(mx, p);
    }

    println!("{}", (mx + 1).saturating_sub(P[0]));
}
