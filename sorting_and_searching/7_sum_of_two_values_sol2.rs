// CSES Sorting & Searching Q-7 :: Sum of Two Values
// DateSolved: 12 April 2026
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
    let [n, x] = array::from_fn(|_| cin.next::<usize>());
    let mut a: Vec<(usize, usize)> = (0..n).map(|i| (cin.next(), i)).collect();
    a.sort_unstable();

    let mut l = 0;
    let mut r = n - 1;
    let mut result = None;
    while l < r {
        let sum = a[l].0 + a[r].0;
        if sum == x {
            result = Some((a[l].1, a[r].1));
            break;
        } else if sum < x {
            l += 1;
        } else {
            r -= 1;
        }
    }

    if let Some((i, j)) = result {
        println!("{} {}", i + 1, j + 1);
    } else {
        println!("IMPOSSIBLE");
    }
}