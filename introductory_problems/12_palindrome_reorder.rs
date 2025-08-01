// CSES Introductory-Problems Q-12 :: Palindrome Reorder
// DateSolved: 17 Jul 2025
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

fn main() {
    let mut cin = Scanner::default();
    let s: String = cin.next();
    let mut f: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *f.entry(c).or_insert(0) += 1;
    }
    let mut no_of_odd = 0;
    for v in f.values() {
        if v % 2 == 1 {
            no_of_odd += 1;
        }
    }
    if no_of_odd > 1 {
        println!("NO SOLUTION");
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
