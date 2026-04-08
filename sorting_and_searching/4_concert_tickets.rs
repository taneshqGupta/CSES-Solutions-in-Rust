// CSES Sorting & Searching Q-4 :: Concert Tickets
// DateSolved: 8 April 2026
// SolvedBy: taneshqGupta

use std::{array, collections::BTreeMap};

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
    let mut h: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        *h.entry(cin.next()).or_insert(0) += 1;
    }
    let t: Vec<usize> = (0..m).map(|_| cin.next()).collect();

    for max_price in t {
        if let Some((&best_price, _)) = h.range(..=max_price).next_back() {
            println!("{}", best_price);
            *h.get_mut(&best_price).unwrap() -= 1;
            if *h.get(&best_price).unwrap() == 0 {
                h.remove(&best_price).unwrap();
            } 
        } else {
            println!("-1");
        }
    }
}