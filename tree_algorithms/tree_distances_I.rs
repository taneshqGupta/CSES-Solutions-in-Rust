// CSES Tree Algorithms - Q-1 :: Tree Distances I
// DateSolved: 13 Sep 2025
// SolvedBy: taneshqGupta

use std::{array, collections::VecDeque};

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

fn bfs(src: usize, tree:  &Vec<Vec<usize>>, dist: &mut Vec<usize>) -> usize {
    let n: usize = tree.len();
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    q.push_back(src);
    used[src] = true;
    dist[src] = 0;
    let mut momo = src;
    while !q.is_empty() {
        momo = q.pop_front().unwrap();
        for &lmao in &tree[momo] {
            if !used[lmao] {
                used[lmao] = true;
                dist[lmao] = dist[momo] + 1;
                q.push_back(lmao);
            }
        }
    }
    return momo;
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let mut tree = vec![vec![]; n];
    for _ in 1..n {
        let [a, b] = array::from_fn(|_| cin.next::<usize>() - 1);
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut dist_0 = vec![0; n];
    let diam1 = bfs(0, &tree, &mut dist_0);
    let mut dist_diam1 = vec![0; n];
    let diam2 = bfs(diam1, &tree, &mut dist_diam1);
    let mut dist_diam2 = vec![0; n];
    bfs(diam2, &tree, &mut dist_diam2);
    for i in 0..n {
        print!("{} ", dist_diam1[i].max(dist_diam2[i]));
    }
    println!();
}
