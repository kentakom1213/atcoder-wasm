#![allow(non_snake_case)]

use cp_library_rs::{
    debug,
    tree::arena::{Arena, ArenaNode, Ptr},
};
use proconio::input;

const SIGMA: usize = 26;

fn main() {
    input! {
        N: usize,
        S: [String; N],
    }

    let mut arena = Arena::<TrieNode>::with_capacity(300_300);
    let root = arena.alloc_default();

    let mut ans: u64 = 0;

    for s in S {
        let mut ptr = root;
        let mut tmp = 0;

        for c in s.chars() {
            let i = ord(c);

            // ptr の i 番目の子
            let ch = if let Some(ch) = arena.get(ptr).children[i] {
                ch
            } else {
                // 追加
                let nch = arena.alloc_default();
                arena.get_mut(ptr).children[i] = Some(nch);
                nch
            };

            let node = arena.get_mut(ch);

            // prefix の数を加算
            tmp += node.val;

            // node の i 番目の子に追加
            node.val += 1;

            ptr = ch;
        }

        debug!(tmp);
        ans += tmp;
    }

    debug_trie(root, &arena, String::new());

    println!("{ans}");
}

fn debug_trie(ptr: Ptr, arena: &Arena<TrieNode>, prefix: String) {
    #[cfg(debug_assertions)]
    {
        let node = arena.get(ptr);
        eprintln!("{prefix} -> {}", node.val);
        for (i, c) in ('a'..='z').enumerate() {
            if let Some(ch) = node.children[i] {
                debug_trie(ch, arena, prefix.clone() + &c.to_string());
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct TrieNode {
    pub children: [Option<Ptr>; SIGMA],
    pub val: u64,
}

impl ArenaNode for TrieNode {}

fn ord(c: char) -> usize {
    c as usize - 'a' as usize
}
