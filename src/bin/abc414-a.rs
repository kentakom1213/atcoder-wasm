#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let (N, L, R) = get!(usize, usize, usize);
    let XY = get!(usize, usize; N);

    println!(
        "{}",
        XY.into_iter().filter(|&(x, y)| x <= L && R <= y).count()
    )
}
