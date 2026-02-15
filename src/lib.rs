/// 入力用マクロ
#[macro_export]
macro_rules! input {
    (parse, $val:expr, usize1) => {(input!(parse, $val, usize) - 1)};
    (parse, $val:expr, chars) => {input!(parse, $val, String).chars().collect::<Vec<_>>()};
    (parse, $val:expr, $t:ty) => {$val.parse::<$t>().unwrap()};
    ($p:tt) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            input!(parse, line.trim(), $p)
    }};
    ($($p:tt),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            let mut iter = line.split_whitespace();
            ( $(input!(parse, iter.next().unwrap(), $p),)* )
    }};
    ($t:tt ; $n:expr) => {(0..$n).map(|_| input!($t)).collect::<Vec<_>>()};
    ($($t:tt),* ; $n:expr) => {(0..$n).map(|_| input!($($t),*)).collect::<Vec<_>>()};
    ($t:tt ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            line.split_whitespace().map(|v| input!(parse, v, $t)).collect::<Vec<_>>()
    }};
    ($t:tt ;; $n:expr) => {(0..$n).map(|_| input!($t ;;)).collect::<Vec<_>>()};
}
