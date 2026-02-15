#![allow(non_snake_case)]

use std::io::{self, Write};

use atcoder_wasm::input;

fn main() {
    let N = input!(usize);
    let A = input!(usize1;;);

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
