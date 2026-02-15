#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let S = input!(chars);
    let n = S.len();

    for i in 1..n {
        if S[i - 1] <= S[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
