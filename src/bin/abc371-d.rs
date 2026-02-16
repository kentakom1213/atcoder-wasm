#![allow(non_snake_case)]

use std::io::{BufWriter, Write, stdout};

use cp_library_rs::{
    algebraic_structure::{operation::Add, to_acted::ToActed},
    data_structure::dynamic_segment_tree::DynamicSegmentTree,
};
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: [i32; N],
        P: [u64; N],
        Q: usize,
        LR: [(i32, i32); Q],
    }

    let mut seg = DynamicSegmentTree::<i32, ToActed<Add<u64>>>::new(-1001001001, 1001001001);

    for (x, p) in X.into_iter().zip(P) {
        seg.update(x, p);
    }

    let mut out = BufWriter::new(stdout().lock());

    for (l, r) in LR {
        let ans = seg.get_range(l..=r);
        writeln!(out, "{ans}").ok();
    }
}
