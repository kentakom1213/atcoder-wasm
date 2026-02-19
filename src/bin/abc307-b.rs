#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [String; N]
    }

    for i in 0..N {
        for j in 0..N {
            let s = S[i].clone() + &S[j];
            if i != j && is_palindrome(&s) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

fn is_palindrome(s: &str) -> bool {
    s.chars().zip(s.chars().rev()).all(|(c, d)| c == d)
}
