#![allow(non_snake_case)]

use atcoder_wasm::input;

fn main() {
    let N = input!(usize);
    let S = input!(chars);

    if N <= 2 {
        println!("0");
        return;
    }

    let mut ans = 0;

    for i in 0..N - 2 {
        match &S[i..i + 3] {
            &['#', '.', '#'] => {
                ans += 1;
            }
            _ => (),
        }
    }

    println!("{ans}");
}
