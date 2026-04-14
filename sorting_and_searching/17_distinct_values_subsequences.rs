// CSES Sorting & Searching Q-17 :: Distinct Values Subsequences
// DateSolved: 15 April 2026
// SolvedBy: taneshqGupta

use std::collections::HashMap;

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

const MOD: u128 = 1e9 as u128 + 7;

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let x: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut mapx: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        *mapx.entry(x[i]).or_insert(0) += 1;
    }
    let mut subseqs = 1;
    for &f in mapx.values() {
        subseqs = (subseqs * (f as u128 + 1)) % MOD;
    }
    println!("{}", (subseqs + MOD - 1) % MOD);
}