#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        D: usize,
        T: [usize; N],
    }

    for (&s, &t) in T.iter().tuple_windows() {
        if t - s <= D {
            println!("{t}");
            return;
        }
    }

    println!("-1");
}
