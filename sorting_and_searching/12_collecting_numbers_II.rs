// CSES Sorting & Searching Q-12 :: Collecting Numbers II
// DateSolved: 14 April 2026
// SolvedBy: taneshqGupta

use std::{array, collections::HashSet};

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
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let mut x: Vec<usize> = vec![0; n];
    let mut position: Vec<usize> = vec![0; n + 1];
    let mut contributes: Vec<bool> = vec![false; n + 1];
    contributes[1] = true;
    for i in 0..n {
        x[i] = cin.next();
        position[x[i]] = i;
    }

    // a new round is required if
    // and only if position of i + 1
    // is less than position of i

    let mut rounds = 1;

    for i in 2..=n {
        if position[i] < position[i - 1] {
            rounds += 1;
            contributes[i] = true;
        }
    }

    for _ in 0..m {
        let [a, b] = array::from_fn(|_| cin.next::<usize>() - 1);
        let (num1, num2) = (x[a], x[b]);
        position[num1] = b;
        position[num2] = a;
        x.swap(a, b);
        let mut setx: HashSet<usize> = HashSet::new();
        setx.insert(num1);
        setx.insert(num2);
        if num1 > 1 {
            setx.insert(num1 - 1);
        }
        if num1 < n {
            setx.insert(num1 + 1);
        }
        if num2 > 1 {
            setx.insert(num2 - 1);
        }
        if num2 < n {
            setx.insert(num2 + 1);
        }
        for num in setx {
            if num == 1 {
                continue;
            }
            if position[num] < position[num - 1] && !contributes[num] {
                contributes[num] = true;
                rounds += 1;
            }
            if position[num] > position[num - 1] && contributes[num] {
                contributes[num] = false;
                rounds -= 1;
            }
        }
        println!("{}", rounds);
    }
}