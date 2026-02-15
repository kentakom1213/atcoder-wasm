#![allow(non_snake_case)]

use std::io::{BufWriter, Write, stdout};

use atcoder_wasm::input;
use cp_library_rs::{
    algebraic_structure::operation::Add, data_structure::segment_tree::SegmentTree,
};

fn main() {
    let (_N, Q) = input!(usize, usize);
    let A = input!(usize;;);

    let mut seg = SegmentTree::<Add<usize>>::from_vec(A);

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..Q {
        let q = input!(usize;;);

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
