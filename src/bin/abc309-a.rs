#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let AB = input!(usize, usize);

    if [(1, 2), (2, 3), (4, 5), (5, 6), (7, 8), (8, 9)].contains(&AB) {
        println!("Yes");
    } else {
        println!("No");
    }
}
