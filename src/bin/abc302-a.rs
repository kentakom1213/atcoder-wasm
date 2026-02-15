#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let (A, B) = get!(u64, u64);

    println!("{}", A.div_ceil(B));
}
