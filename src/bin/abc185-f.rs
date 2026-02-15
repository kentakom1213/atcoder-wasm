#![allow(non_snake_case)]

use std::io::{BufWriter, Write, stdout};

use atcoder_wasm::get;
use cp_library_rs::{
    algebraic_structure::operation::Xor, data_structure::segment_tree::SegmentTree,
};

fn main() {
    let (_N, Q) = get!(usize, usize);
    let A = get!(usize;;);

    let mut seg = SegmentTree::<Xor>::from_vec(A);

    let mut out = BufWriter::new(stdout().lock());

    for _ in 0..Q {
        let (t, x, y) = get!(usize, usize, usize);

        if t == 1 {
            *seg.get_mut(x - 1).unwrap() ^= y;
        } else {
            let ans = seg.get_range(x - 1..y);

            writeln!(out, "{ans}").ok();
        }
    }
}
