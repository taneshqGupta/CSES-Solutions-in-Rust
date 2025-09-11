// CSES Introductory-Problems Q-15 :: Creating Strings
// DateSolved: 11 Sep 2025
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

fn next_permutation<T: Ord>(a: &mut Vec<T>) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }
    let Some(p) = (0..n - 1).rfind(|&i| a[i] < a[i + 1]) else {
        a.reverse();
        return false;
    };
    let k = (p + 1..n).rfind(|&i| a[i] > a[p]).unwrap();
    a.swap(p, k);
    a[p + 1..].reverse();
    return true;
}

fn main() {
    let mut cin = Scanner::default();
    let mut s: Vec<char> = cin.next::<String>().chars().collect();
    let mut setx: BTreeSet<String> = BTreeSet::new();
    s.sort();
    setx.insert(s.iter().collect());
    while next_permutation(&mut s) {
        setx.insert(s.iter().collect());
    }
    println!("{}", setx.len());
    for c in setx {
        println!("{}", c);
    }
}
