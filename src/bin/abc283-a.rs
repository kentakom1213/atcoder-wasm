#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let (A, B) = get!(u64, u32);

    println!("{}", A.pow(B));
}
