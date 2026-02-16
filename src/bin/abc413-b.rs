#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let N = get!(usize);
    let S = get!(String; N);

    let mut res = vec![];

    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            res.push(S[i].clone() + &S[j]);
        }
    }

    res.sort_unstable();
    res.dedup();

    println!("{}", res.len());
}
