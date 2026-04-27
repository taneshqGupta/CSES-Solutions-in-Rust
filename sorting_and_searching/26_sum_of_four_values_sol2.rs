// CSES Sorting & Searching Q-26 :: Sum of 4 Values
// DateSolved: 27 April 2026
// SolvedBy: taneshqGupta

use std::collections::BTreeSet;

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
    let n: usize = cin.next();
    let x: usize = cin.next();
    let a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut sums = BTreeSet::new();

    let mut ans = None;
    'outer: for i in 0..n {
        for j in i + 1..n {
            let sumx = a[i] + a[j];
            if sumx >= x {
                continue;
            }
            let target = x - sumx;
            if let Some((_, p, q)) = sums.range((target, 0, 0)..(target, i, i)).next() {
                ans = Some((i + 1, j + 1, p + 1, q + 1));
                break 'outer;
            }
        }
        for j in 0..i {
            sums.insert((a[i] + a[j], i, j));
        }
    }

    if let Some((a, b, c, d)) = ans {
        println!("{} {} {} {}", a, b, c, d);
    } else {
        println!("IMPOSSIBLE");
    }
}