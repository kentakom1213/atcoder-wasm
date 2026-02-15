#![allow(non_snake_case)]

use std::io::{Write, stdout};

use cp_library_rs::get;

fn main() {
    let N = get!(usize);
    let SS = get!(String; N);
    let (X, Y) = get!(usize1, String);

    let mut out = stdout();

    if SS[X] == Y {
        writeln!(out, "Yes").ok();
    } else {
        writeln!(out, "No").ok();
    }
}
