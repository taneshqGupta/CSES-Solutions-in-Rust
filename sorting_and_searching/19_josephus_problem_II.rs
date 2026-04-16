// CSES Sorting & Searching Q-18 :: Josephus Problem II
// DateSolved: 16 April 2026
// SolvedBy: taneshqGupta

use std::array;

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

#[derive(Debug)]
struct FenwickTree {
    alive: Vec<bool>,
    tree: Vec<usize>,
    n: usize,
    total_alive: usize,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            alive: vec![false; n],
            tree: vec![0; n],
            n,
            total_alive: 0,
        }
    }

    fn add(&mut self, mut i: usize) {
        self.alive[i] = true;
        while i < self.n {
            self.tree[i] += 1;
            i |= i + 1;
        }
        self.total_alive += 1;
    }

    fn remove(&mut self, mut i: usize) {
        self.alive[i] = false;
        while i < self.n {
            self.tree[i] -= 1;
            i |= i + 1;
        }
        self.total_alive -= 1;
    }

    fn alive_till(&self, mut i: usize) -> usize {
        let mut sum = 0;
        loop {
            sum += self.tree[i];
            let g_i = i & (i + 1);
            if g_i == 0 {
                break;
            }
            i = g_i - 1;
        }
        sum
    }
}

fn main() {
    let mut cin = Scanner::default();
    let [n, k] = array::from_fn(|_| cin.next::<usize>());

    let mut ring = FenwickTree::new(n);
    for i in 0..n {
        ring.add(i);
    }

    let mut logical_pos = 0;

    while ring.total_alive > 0 {
        logical_pos = (logical_pos + k) % ring.total_alive;

        let target = logical_pos + 1;

        let mut low = 0;
        let mut high = n - 1;
        let mut ans = 0;

        while low <= high {
            let mid = low + (high - low) / 2;
            if ring.alive_till(mid) >= target {
                ans = mid;
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        print!("{} ", ans + 1);
        ring.remove(ans);
        dbg!(&ring);
    }
    println!();
}