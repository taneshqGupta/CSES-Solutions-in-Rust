// CSES Sorting & Searching Q-16 :: Distinct Values Subarrays
// DateSolved: 14 April 2026
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
    let x: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut subarrays = 1;
    let mut setx: HashSet<usize> = HashSet::new();
    setx.insert(x[0]);
    let mut startpoint: usize = 0;
    for i in 1..n {
        // dbg!(&startpoint, &setx);
        while setx.contains(&x[i]) {
            setx.remove(&x[startpoint]);
            startpoint += 1;
        }
        setx.insert(x[i]);
        subarrays += i - startpoint + 1;
    }
    println!("{}", subarrays);
}