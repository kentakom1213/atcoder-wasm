#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String
    }

    let mut open = 0;
    let mut res = String::new();

    for c in S.chars() {
        match c {
            '(' => {
                open += 1;
                res.push(c);
            }
            ')' => {
                if open >= 1 {
                    while let Some(x) = res.pop() {
                        if x == '(' {
                            open -= 1;
                            break;
                        }
                    }
                } else {
                    res.push(c);
                }
            }
            _ => {
                res.push(c);
            }
        }
    }

    println!("{res}");
}
