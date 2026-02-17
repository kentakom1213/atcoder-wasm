#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        _N: usize,
        K: usize,
        S: String
    }

    let mut cnt = 0;
    let mut ans = String::new();

    for c in S.chars() {
        if c == 'x' {
            ans.push('x');
        } else {
            if cnt < K {
                ans.push('o');
                cnt += 1;
            } else {
                ans.push('x');
            }
        }
    }

    println!("{ans}");
}
