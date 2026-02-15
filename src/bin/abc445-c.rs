#![allow(non_snake_case)]

use std::io::{self, Write};

use cp_library_rs::get;

fn main() {
    let N = get!(usize);
    let A = get!(usize1;;);

    let mut ans = vec![0; N];

    for i in (0..N).rev() {
        if ans[i] != 0 {
            continue;
        }
        if A[i] == i {
            ans[i] = i + 1;
        } else {
            ans[i] = ans[A[i]];
        }
    }

    let mut out = io::stdout();

    for i in 0..N {
        writeln!(out, "{}", ans[i]).ok();
    }
}
