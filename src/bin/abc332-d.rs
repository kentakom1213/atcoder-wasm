#![allow(non_snake_case)]

use std::collections::VecDeque;

use proconio::input;
use rustc_hash::FxHashSet;

fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[u32; W]; H],
        B: [[u32; W]; H],
    }

    let mut visited = FxHashSet::default();
    let mut q = VecDeque::new();
    q.push_back((A, 0));

    while let Some((arr, d)) = q.pop_front() {
        if &arr == &B {
            println!("{d}");
            return;
        }
        for r in 0..H - 1 {
            let new = swap_row(&arr, r);
            if visited.contains(&new) {
                continue;
            }
            visited.insert(new.clone());
            q.push_back((new, d + 1));
        }
        for c in 0..W - 1 {
            let new = swap_col(&arr, c);
            if visited.contains(&new) {
                continue;
            }
            visited.insert(new.clone());
            q.push_back((new, d + 1));
        }
    }

    println!("-1");
}

fn swap_row(arr: &Vec<Vec<u32>>, r: usize) -> Vec<Vec<u32>> {
    let mut res = arr.clone();
    res.swap(r, r + 1);
    res
}

fn swap_col(arr: &Vec<Vec<u32>>, c: usize) -> Vec<Vec<u32>> {
    let mut res = arr.clone();
    for row in res.iter_mut() {
        row.swap(c, c + 1);
    }
    res
}
