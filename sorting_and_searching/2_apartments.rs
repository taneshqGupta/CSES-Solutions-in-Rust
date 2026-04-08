// CSES Sorting & Searching Q-2 :: Apartments
// DateSolved: 8 April 2026
// SolvedBy: taneshqGupta

use std::array;

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
    let [n, m, k] = array::from_fn(|_| cin.next::<usize>());
    let mut a: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    let mut b: Vec<usize> = (0..m).map(|_| cin.next()).collect();

    a.sort_unstable();
    b.sort_unstable();

    let mut i = 0;
    let mut j = 0;
    let mut count = 0;

    while i < n && j < m {
        if b[j] + k >= a[i] && b[j] <= a[i] + k {
            count += 1;
            i += 1;
            j += 1;
            continue;
        }
        if b[j] + k < a[i] { // apartment too small then the smallest requirement
            // must skip this apartment
            j += 1;
            continue;
        }
        if b[j] > a[i] + k { // requirement too low for the smallest apartment
            // must skip this candidate
            i += 1;
            continue;
        } 
    }

    println!("{}", count);
}
