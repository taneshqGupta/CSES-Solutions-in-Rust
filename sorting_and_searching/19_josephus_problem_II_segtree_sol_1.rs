// CSES Sorting & Searching Q-19 :: Josephus Problem II
// DateSolved: 18 April 2026
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
struct SegTree {
    a: Vec<usize>,
    t: Vec<usize>,
}

impl SegTree {
    fn new(a: Vec<usize>) -> Self {
        let n = a.len();
        SegTree {
            a,
            t: vec![0; 4 * n + 1],
        }
    }
    fn init_build(&mut self, v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.t[v] = self.a[tl];
        } else {
            let tm = tl + (tr - tl) / 2;
            self.init_build(v * 2, tl, tm);
            self.init_build(v * 2 + 1, tm + 1, tr);
            self.t[v] = self.t[2 * v] + self.t[2 * v + 1];
        }
    }
    fn range_sum(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> usize {
        if tl == l && tr == r {
            return self.t[v];
        }
        let tm = tl + (tr - tl) / 2;
        if r <= tm {
            return self.range_sum(v * 2, tl, tm, l, r);
        }
        if l > tm {
            return self.range_sum(v * 2 + 1, tm + 1, tr, l, r);
        }
        self.range_sum(v * 2, tl, tm, l, tm) + self.range_sum(v * 2 + 1, tm + 1, tr, tm + 1, r)
    }
    fn update(&mut self, v: usize, tl: usize, tr: usize, i: usize, new_val: usize) {
        if tl == tr {
            self.t[v] = new_val;
        } else {
            let tm = tl + (tr - tl) / 2;
            if i <= tm {
                self.update(v * 2, tl, tm, i, new_val);
            } else {
                self.update(v * 2 + 1, tm + 1, tr, i, new_val);
            }
            self.t[v] = self.t[v * 2] + self.t[v * 2 + 1];
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let [n, k] = array::from_fn(|_| cin.next::<usize>());
    let a = vec![1; n];
    let mut ring = SegTree::new(a);
    ring.init_build(1, 0, n - 1);
    // dbg!(&ring);

    let mut curr = 0;
    while ring.t[1] > 0 {
        curr = (curr + k) % ring.t[1];
        let target = curr + 1;
        let mut low = 0;
        let mut high = n - 1;
        let mut ans = 0;
        while low <= high {
            let mid = low + (high - low) / 2;
            if ring.range_sum(1, 0, n - 1, 0, mid) < target {
                low = mid + 1;
            } else {
                ans = mid;
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
        print!("{} ", ans + 1);
        ring.update(1, 0, n - 1, ans, 0);
    }
    println!();
}