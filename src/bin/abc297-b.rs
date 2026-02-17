#![allow(non_snake_case)]

use cp_library_rs::utils::boolutil::BoolUtil;
use proconio::input;

fn main() {
    input! {
        S: String
    }

    let mut bpos = vec![];
    let mut rpos = vec![];
    let mut kpos = vec![];
    for (i, c) in S.chars().enumerate() {
        if c == 'B' {
            bpos.push(i);
        }
        if c == 'R' {
            rpos.push(i);
        }
        if c == 'K' {
            kpos.push(i);
        }
    }
    let isok_1 = (bpos[0] ^ bpos[1]) & 1 == 1;
    let isok_2 = rpos[0] < kpos[0] && kpos[0] < rpos[1];

    println!("{}", (isok_1 && isok_2).yesno());
}
