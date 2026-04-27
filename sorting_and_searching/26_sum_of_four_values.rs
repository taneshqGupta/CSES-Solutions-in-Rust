// CSES Sorting & Searching Q-26 :: Sum of 4 Values
// DateSolved: 27 April 2026
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
    let n: usize = cin.next();
    let x: usize = cin.next();
    let a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut sums: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for i in 1..n {
        for j in 0..i {
            sums.entry(a[i] + a[j]).or_insert(vec![]).push((i, j));
        }
    }
    // dbg!(&sums);
    let mut ans = None;
    'outer: for i in 1..n {
        for j in 0..i {
            let sumx = a[i] + a[j];
            if sumx >= x {
                continue;
            }
            let target = x - sumx;
            if let Some(vecx) = sums.get(&target) {
                for &(p, q) in vecx {
                    if i == p || j == p || i == q || j == q {
                        continue;
                   }
                   ans = Some((i + 1, j + 1, p + 1, q + 1));
                   break 'outer;
                }
            }
        }
    }
    if let Some((a, b, c, d)) = ans {
        println!("{} {} {} {}", a, b, c, d);
    } else {
        println!("IMPOSSIBLE");
    }
    
}