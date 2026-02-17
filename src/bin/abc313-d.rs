#![allow(non_snake_case)]

use cp_library_rs::{debug, get};
use itertools::Itertools;

fn main() {
    let (N, K) = get!(usize, usize);

    let mut A: Vec<u8> = vec![0; N + 1];

    // K+1 回かけて A[1]..A[K+1] の値を特定
    let mut q: Vec<usize> = (2..=K + 1).collect();

    for i in 1..=K + 1 {
        println!("? {}", q.iter().join(" "));
        let t = get!(u8);

        for &j in &q {
            A[j] ^= t;
        }

        if i > K {
            break;
        }

        q[i - 1] -= 1;
    }

    debug!(A);

    // A[1]..A[K-1] から残りの部分を特定
    let offset = (1..=K - 1).fold(0, |acc, i| acc ^ A[i]);
    for i in K + 2..=N {
        q.pop();
        q.push(i);
        println!("? {}", q.iter().join(" "));
        let t = get!(u8);

        A[i] = t ^ offset;
    }

    println!("! {}", A[1..].iter().join(" "));
}
