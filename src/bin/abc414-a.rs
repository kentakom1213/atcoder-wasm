#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let (N, L, R) = input!(usize, usize, usize);
    let XY = input!(usize, usize; N);

    println!(
        "{}",
        XY.into_iter().filter(|&(x, y)| x <= L && R <= y).count()
    )
}
