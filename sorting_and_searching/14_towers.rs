// CSES Sorting & Searching Q-14 :: Towers
// DateSolved: 14 April 2026
// SolvedBy: taneshqGupta

use std::{
    collections::BTreeMap,
    ops::Bound::{Excluded, Unbounded},
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
    let n: usize = cin.next();
    let a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut tops: BTreeMap<usize, usize> = BTreeMap::new();
    let mut towers = 1;
    tops.insert(a[0], 1);
    for i in 1..n {
        let lowest = tops.range((Excluded(&a[i]), Unbounded)).next();
        if let Some((&mytop, &_count)) = lowest {
            *tops.get_mut(&mytop).unwrap() -= 1;
            if *tops.get(&mytop).unwrap() == 0 {
                tops.remove(&mytop);
            }
            *tops.entry(a[i]).or_insert(0) += 1;
            // println!("inserting {} on top of {}", a[i], mytop);
        } else {
            *tops.entry(a[i]).or_insert(0) += 1;
            towers += 1;
            // println!("making new tower from {}", a[i]);
        }
        // dbg!(&tops);
    }
    println!("{}", towers);
}