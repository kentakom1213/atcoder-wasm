pub mod data_structure;

/// 入力用マクロ
#[macro_export]
macro_rules! get {
    (parse, $val:expr, usize1) => {(get!(parse, $val, usize) - 1)};
    (parse, $val:expr, chars) => {get!(parse, $val, String).chars().collect::<Vec<_>>()};
    (parse, $val:expr, $t:ty) => {$val.parse::<$t>().unwrap()};
    ($p:tt) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            get!(parse, line.trim(), $p)
    }};
    ($($p:tt),*) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            let mut iter = line.split_whitespace();
            ( $(get!(parse, iter.next().unwrap(), $p),)* )
    }};
    ($t:tt ; $n:expr) => {(0..$n).map(|_| get!($t)).collect::<Vec<_>>()};
    ($($t:tt),* ; $n:expr) => {(0..$n).map(|_| get!($($t),*)).collect::<Vec<_>>()};
    ($t:tt ;;) => {{
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            line.split_whitespace().map(|v| get!(parse, v, $t)).collect::<Vec<_>>()
    }};
    ($t:tt ;; $n:expr) => {(0..$n).map(|_| get!($t ;;)).collect::<Vec<_>>()};
}
