#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let S = get!(chars);
    let n = S.len();

    for i in 1..n {
        if S[i - 1] <= S[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
