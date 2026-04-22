// CSES Sorting & Searching Q-19 :: Josephus Problem II
// DateSolved: 22 April 2026
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
    a: Vec<usize>,
    bit: Vec<usize>,
    n: usize,
    total_sum: usize,
}

impl FenwickTree {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        let mut bit: Vec<usize> = vec![0; n];
        let mut total_sum = 0;
        for i in 0..n {
            let mut j_i = i;
            while j_i < n {
                bit[j_i] += a[i];
                j_i |= j_i + 1;
            }
            total_sum += a[i];
        }
        FenwickTree {
            a,
            bit,
            n,
            total_sum,
        }
    }
    fn update(&mut self, i: usize, new_val: usize) {
        let mut j_i = i;
        let old_val = self.a[i];
        while j_i < self.n {
            self.bit[j_i] += new_val;
            self.bit[j_i] -= old_val;
            j_i |= j_i + 1;
        }
        self.a[i] = new_val;
        self.total_sum += new_val;
        self.total_sum -= old_val;
    }
    fn _sum(&self, mut i: usize) -> usize {
        let mut sum = 0;
        loop {
            sum += self.bit[i];
            let g_i = i & (i + 1);
            if g_i == 0 {
                break;
            }
            i = g_i - 1;
        }
        sum
    }
    /// returns the largest index with prefix_sum >= k.
    fn binary_lift_search(&self, k: usize) -> usize {
        let mut current = 0;
        let mut sum = 0;
        for jump in (0..=self.n.ilog2()).rev() {
            let next_position = current + (1 << jump);
            if next_position > self.n {
                continue;
            }
            if sum + self.bit[next_position - 1] < k {
                sum += self.bit[next_position - 1];
                current = next_position;
            }
        }
        current
    }
}

fn main() {
    let mut cin = Scanner::default();
    let [n, k] = array::from_fn(|_| cin.next::<usize>());
    let a = vec![1; n];
    let mut ring = FenwickTree::new(a);

    let mut logical_pos = 0;

    while ring.total_sum > 0 {
        logical_pos = (logical_pos + k) % ring.total_sum;
        let ans = ring.binary_lift_search(logical_pos + 1);

        print!("{} ", ans + 1);
        ring.update(ans, 0);
    }
    println!();
}