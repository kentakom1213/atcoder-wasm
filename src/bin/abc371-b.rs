#![allow(non_snake_case)]

use cp_library_rs::utils::boolutil::BoolUtil;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, char); M]
    }

    let mut hasM = vec![false; N];

    for (a, b) in AB {
        if b == 'F' {
            println!("No");
            continue;
        }
        println!("{}", (!hasM[a]).yesno());
        hasM[a] |= b == 'M';
    }
}
