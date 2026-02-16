#![allow(non_snake_case)]

use std::collections::VecDeque;

use cp_library_rs::debug;
use proconio::input;

fn main() {
    input! {
        Q: usize
    }

    let mut q = VecDeque::new();

    for _ in 0..Q {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                c: u64,
                x: u64
            }
            q.push_back((c, x));
        } else {
            input! {
                k: u64
            }
            let mut sum: u64 = 0;
            let mut cnt: u64 = 0;
            while let Some(&(c, x)) = q.front() {
                if cnt + c <= k {
                    q.pop_front();
                    cnt += c;
                    sum += c * x;
                } else if cnt + c > k {
                    let (c, x) = q.front_mut().unwrap();
                    let pop = k - cnt;
                    *c -= pop;
                    cnt = k;
                    sum += pop * *x;
                }
                if cnt == k {
                    break;
                }
            }

            debug!(q);
            println!("{sum}");
        }
    }
}
