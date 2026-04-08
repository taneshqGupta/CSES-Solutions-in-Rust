// CSES Sorting & Searching Q-1 :: Distinct Numbers
// DateSolved: 8 April 2026
// SolvedBy: taneshqGupta

use std::collections::HashSet;

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
    let mut setx: HashSet<usize> = HashSet::new();
    for _ in 0..n {
        setx.insert(cin.next());
    }
    println!("{}", setx.len());
}
