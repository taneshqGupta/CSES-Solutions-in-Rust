// CSES ProblemSet Q-12 :: Palindrome Reorder
// DateSolved: 17 Jul 2025
// SolvedBy: taneshqGupta

use std::collections::HashMap;

struct Scanner(Vec<String>);
impl Scanner {
    fn new() -> Self {
        let input = std::io::read_to_string(std::io::stdin()).unwrap();
        Scanner(input.split_whitespace().map(String::from).rev().collect())
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.0.pop().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut cin = Scanner::new();
    let s: String = cin.next();
    let mut f: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *f.entry(c).or_insert(0) += 1;
    }
    // dbg!(&freq);
    let mut no_of_odd = 0;
    for v in f.values() {
        if v % 2 == 1 {
            no_of_odd += 1;
        }
    }
    if no_of_odd > 1 {
        println!("{}", "NO SOLUTION");
        return;
    }
    let mut a = vec![];
    let mut b = vec![];
    for (&k, &v) in f.iter() {
        if v % 2 == 0 {
            for _ in 0..(v / 2) {
                a.push(k);
                b.push(k);
            }
        }
    }
    for (&k, &v) in f.iter() {
        if v % 2 == 1 {
            for _ in 0..v {
                a.push(k);
            }
        }
    }
    for c in a.iter().chain(b.iter().rev()) {
        print!("{}", c);
    }
    println!();
}
