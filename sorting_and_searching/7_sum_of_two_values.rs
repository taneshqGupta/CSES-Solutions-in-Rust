// CSES Sorting & Searching Q-7 :: Sum of Two Values
// DateSolved: 12 April 2026
// SolvedBy: taneshqGupta

use std::{array, collections::BTreeSet};

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
    let [n, x] = array::from_fn(|_| cin.next::<usize>());
    let mut a: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..n {
        a.insert((cin.next(), i));
    }
    let mut result: Option<(usize, usize)> = None;
    for &(val, i) in a.iter() {
        if val >= x {
            continue;
        }
        let diff = x - val;
        if a.range((diff, 0)..=(diff, n)).count() == 0 {
            continue;
        }
        for &(_val2, i2) in a.range((diff, 0)..=(diff, n)) {
            if i == i2 {
                continue;
            } else {
                result = Some((i, i2));
                break;
            }
        }
        if result.is_some() {
            break;
        }
    }
    if result.is_none() {
        println!("IMPOSSIBLE");
    } else {
        println!("{} {}", result.unwrap().0 + 1, result.unwrap().1 + 1);
    }
}