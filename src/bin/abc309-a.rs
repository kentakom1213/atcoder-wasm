#![allow(non_snake_case)]

use atcoder_wasm::get;

fn main() {
    let AB = get!(usize, usize);

    if [(1, 2), (2, 3), (4, 5), (5, 6), (7, 8), (8, 9)].contains(&AB) {
        println!("Yes");
    } else {
        println!("No");
    }
}
