#![allow(non_snake_case)]

use std::collections::HashMap;

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let mut cnt = HashMap::new();

    for c in S.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }

    debug!(cnt);

    for (&k, &v) in &cnt {
        if v == 1 {
            let (_, ans) = S.chars().zip(1..).find(|&(c, _)| c == k).unwrap();

            println!("{ans}")
        }
    }
}
