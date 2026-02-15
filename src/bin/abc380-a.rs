#![allow(non_snake_case)]

use atcoder_wasm::get;

fn main() {
    let N = get!(String);
    let mut cnt = [0; 3];

    for c in N.chars() {
        if c == '1' {
            cnt[0] += 1;
        }
        if c == '2' {
            cnt[1] += 1;
        }
        if c == '3' {
            cnt[2] += 1;
        }
    }

    if cnt == [1, 2, 3] {
        println!("Yes");
    } else {
        println!("No");
    }
}
