#![allow(non_snake_case)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x: char,
        y: char,
        z: char,
    }

    for v in (0..3).permutations(3) {
        let (a, b, c) = (v[0], v[1], v[2]);
        let ab = (x == '<') ^ (a < b);
        let ac = (y == '<') ^ (a < c);
        let bc = (z == '<') ^ (b < c);

        if ab && ac && bc {
            let mut ord = [(a, 'A'), (b, 'B'), (c, 'C')];
            ord.sort_unstable();

            println!("{}", ord[1].1);
            return;
        }
    }

    unreachable!()
}
