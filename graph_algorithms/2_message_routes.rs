// CSES Graph-Algorithms Q-2 :: Message Routes
// DateSolved: 9 Sep 2025
// SolvedBy: taneshqGupta

use std::{
    array,
    collections::VecDeque,
    // io::{stderr, Write},
};

#[derive(Default, Debug)]
struct Scanner(Vec<String>);
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(c) = self.0.pop() {
                return c.parse().ok().unwrap();
            }
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            self.0 = s.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let mut store = vec![vec![]; n + 1];
    for _ in 0..m {
        let [a, b] = array::from_fn(|_| cin.next::<usize>());
        store[a].push(b);
        store[b].push(a);
    }
    let mut q = VecDeque::new();
    q.push_back(1);
    let mut used = vec![false; n + 1];
    used[1] = true;
    let mut parents = vec![0; n + 1];
    let mut distances = vec![0; n + 1];
    let mut ys = false;
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        // dbg!(&momo, &q, &used, &parents, &distances);
        // let _ = writeln!(stderr());
        if momo == n {
            ys = true;
            break;
        }
        for &comp in &store[momo] {
            if !used[comp] {
                used[comp] = true;
                parents[comp] = momo;
                distances[comp] = distances[momo] + 1;
                q.push_back(comp);
            }
        }
        // dbg!(&momo, &q, &used, &parents, &distances);
        // let _ = writeln!(stderr());
        // let _ = writeln!(stderr());
    }
    if !ys {
        println!("IMPOSSIBLE");
    }
    else {
        println!("{}", distances[n] + 1);
        let mut moves = Vec::new();
        let mut curr = n;
        moves.push(curr);
        while curr != 1 {
            moves.push(parents[curr]);
            curr = parents[curr];
        }
        moves.reverse();
        for &mov in &moves {
            print!("{} ", mov);
        }
        println!();
    }
}
