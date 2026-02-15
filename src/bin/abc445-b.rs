#![allow(non_snake_case)]

use cp_library_rs::get;

fn main() {
    let N = get!(usize);
    let SS = get!(String; N);

    let m = SS.iter().map(|s| s.len()).max().unwrap();

    for s in SS {
        let p = m - s.len();
        let pad = ".".repeat(p / 2);
        println!("{pad}{s}{pad}");
    }
}
