// CSES Tree Algorithms - Q-2 :: Tree Distances II
// DateSolved: 16 Sep 2025
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

fn dfs(curr: usize, parent: usize, tree: &Vec<Vec<usize>>, subtree_sizes: &mut Vec<usize>) {
    subtree_sizes[curr] = 1;
    for &child in &tree[curr] {
        if child != parent {
            dfs(child, curr, tree, subtree_sizes);
            subtree_sizes[curr] += subtree_sizes[child];
        }
    }
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
    let mut subtree_sizes = vec![0; n];
    dfs(0, 0, &tree, &mut subtree_sizes);
    // dbg!(&subtree_sizes);
    let mut dist_from_root = vec![0; n];
    let mut used = vec![false; n];
    used[0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        // dbg!(&momo, &used, &dist_from_root);
        for &child in &tree[momo] {
            if !used[child] {
                used[child] = true;
                q.push_back(child);
                dist_from_root[child] = dist_from_root[momo] + 1;
            }
        }
        // dbg!(&momo, &used, &dist_from_root);
    }
    // dbg!(dist_from_root);
    let mut sums = vec![0; n];
    sums[0] = dist_from_root.iter().sum();
    let mut used = vec![false; n];
    used[0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        // dbg!(&momo, &used, &dist_from_root);
        for &child in &tree[momo] {
            if !used[child] {
                used[child] = true;
                q.push_back(child);
                sums[child] = sums[momo] + (n - subtree_sizes[child] - 1) - (subtree_sizes[child] - 1);
            }
        }
        // dbg!(&momo, &used, &dist_from_root);
    }
    // dbg!(&sums);
    for &sum in &sums {
        print!("{} ", sum);
    }
    println!();
}
