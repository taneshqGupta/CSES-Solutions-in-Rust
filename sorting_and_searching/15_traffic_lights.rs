// CSES Sorting & Searching Q-15 :: Traffic Lights
// DateSolved: 14 April 2026
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
    let [x, n] = array::from_fn(|_| cin.next::<usize>());
    let mut positions: BTreeMap<usize, usize> = BTreeMap::new();
    let mut lengths: BTreeMap<usize, usize> = BTreeMap::new();
    positions.insert(0, x);
    lengths.insert(x, 1);
    for _ in 0..n {
        let new_position: usize = cin.next();
        let (&start, &len) = positions.range(..new_position).next_back().unwrap();
        let new_interval_len = start + len - new_position;
        positions.insert(new_position, new_interval_len);
        *positions.get_mut(&start).unwrap() -= new_interval_len;
        *lengths.get_mut(&len).unwrap() -= 1;
        if *lengths.get(&len).unwrap() == 0 {
            lengths.remove(&len);
        }
        *lengths.entry(new_interval_len).or_insert(0) += 1;
        *lengths.entry(len - new_interval_len).or_insert(0) += 1;
        let ans = lengths.last_key_value().unwrap().0;
        println!("{}", ans);
    }
}