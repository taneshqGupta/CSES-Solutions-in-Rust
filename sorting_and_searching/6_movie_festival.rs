// CSES Sorting & Searching Q-6 :: Movie Festival
// DateSolved: 12 April 2026
// SolvedBy: taneshqGupta

use std::collections::{BTreeMap, HashMap, HashSet};

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
    let mut times: Vec<(usize, usize)> = Vec::with_capacity(n);
    for _ in 0..n {
        let startime = cin.next();
        let endtime = cin.next();
        times.push((endtime, startime));
    }
    times.sort_unstable();
    let mut last_chosen_endtime = 0;
    let mut ans = 0;
    for (j, i) in times {
        if i < last_chosen_endtime {
            continue;
        } else {
            last_chosen_endtime = j;
            ans += 1;
        }
    }
    println!("{}", ans);
}