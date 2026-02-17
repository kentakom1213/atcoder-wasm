#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M]
    }

    let G = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[b].push(a);
        g
    });

    // 最強の候補を見つける
    let mut p = 0;
    while !G[p].is_empty() {
        p = G[p][0];
    }

    // p から到達性判定
    let H = AB.iter().fold(vec![vec![]; N], |mut g, &(a, b)| {
        g[a].push(b);
        g
    });

    let mut isok = vec![false; N];
    let mut q = vec![p];
    while let Some(u) = q.pop() {
        isok[u] = true;
        for &v in &H[u] {
            if isok[v] {
                continue;
            }
            q.push(v);
            isok[v] = true;
        }
    }

    if isok.into_iter().all(|x| x) {
        println!("{}", p + 1);
    } else {
        println!("-1");
    }
}
