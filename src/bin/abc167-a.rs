#![allow(non_snake_case)]

use atcoder_wasm::input;
use cp_library_rs::utils::boolutil::BoolUtil;

fn main() {
    let S = input!(String);
    let mut T = input!(String);

    T.pop();

    println!("{}", (S == T).yesno());
}
