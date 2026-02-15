fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();

    let s: Vec<_> = s.trim().chars().collect();
    let n = s.len();

    if s[0] == s[n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
