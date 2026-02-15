#![allow(non_snake_case)]

use cp_library_rs::get;
use cp_library_rs::utils::boolutil::BoolUtil;

fn main() {
    let S = get!(String);
    let mut T = get!(String);

    T.pop();

    println!("{}", (S == T).yesno());
}
