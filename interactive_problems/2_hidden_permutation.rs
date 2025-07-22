// CSES ProblemSet Q-2 :: Hidden Permutation
// DateSolved: 23 Jul 2025
// SolvedBy: taneshqGupta

use std::collections::BTreeMap;

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
    let mut a: Vec<usize> = (1..=n).map(|i| i).collect();
    // dbg!(&a);
    for i in 1..n {
        let mut minp = 0;
        let mut maxp = i;
        while minp != maxp {
            let comparable = minp + (maxp - minp) / 2;
            println!("? {} {}", a[comparable], a[i]);
            let s: String = cin.next();
            if s == "YES" {
                minp = comparable + 1;
            } else {
                maxp = comparable;
            }
        }
        if i == minp {
            // dbg!(&a);
            continue;
        }
        let mut pivot = i;
        if pivot > 0 {
            while pivot != minp {
                a.swap(pivot, pivot - 1);
                pivot -= 1;
            }
        }
        // dbg!(&a);
    }
    let mut mapx = BTreeMap::new();
    for (i, j) in a.iter().zip(1..=n) {
        mapx.insert(i, j);
    }
    print!("! ");
    for i in mapx.values() {
        print!("{} ", i)
    }
    println!();
}
