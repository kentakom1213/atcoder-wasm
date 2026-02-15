#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let N = get!(usize);
    let S = get!(String; N);

    for s in S.iter().rev() {
        println!("{s}");
    }
}
