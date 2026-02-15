#![allow(non_snake_case)]

use std::io::{Write, stdout};

use atcoder_wasm::{data_structure::segment_tree::SegmentTree, get};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let A = get!(usize;;);

    let mut seg = SegmentTree::build(&A, 0, |a, b| a + b);

    let mut out = stdout();

    for _ in 0..Q {
        let q = get!(usize;;);

        if q[0] == 1 {
            let x = q[1];

            let p = seg[x - 1];
            let q = seg[x];

            seg.update(x - 1, q);
            seg.update(x, p);
        } else {
            let l = q[1];
            let r = q[2];

            let ans = seg.get_range(l - 1..r);

            writeln!(out, "{ans}").ok();
        }
    }
}
