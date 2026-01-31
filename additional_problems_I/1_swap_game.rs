// CSES Additional Problems I - Q-1 :: Swap Game
// DateSolved: 11 Sep 2025
// SolvedBy: taneshqGupta

use std::collections::{HashSet, VecDeque};

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
    let mut inp: usize = 0;
    for _ in 0..9 {
        let digit = cin.next::<usize>();
        inp = inp * 10 + digit;
    }

    let swappable = vec![
        (0, 1),
        (0, 3),
        (1, 2),
        (1, 4),
        (2, 5),
        (3, 4),
        (3, 6),
        (4, 5),
        (4, 7),
        (5, 8),
        (6, 7),
        (7, 8),
    ];

    let powers = [
        1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000,
    ];

    let base: usize = 123456789;
    let mut q = VecDeque::new();
    let mut used = HashSet::new();
    q.push_back((base, 0));
    used.insert(base);

    while let Some((momo, d)) = q.pop_front() {
        if momo == inp {
            println!("{}", d);
            return;
        }
        for &(i, j) in &swappable {
            let ipow = powers[i];
            let jpow = powers[j];
            let x = (momo / ipow) % 10;
            let y = (momo / jpow) % 10;
            let mut newx = momo;
            newx = newx - x * ipow;
            newx = newx + y * ipow;
            newx = newx - y * jpow;
            newx = newx + x * jpow;
            if !used.contains(&newx) {
                q.push_back((newx, d + 1));
                used.insert(newx);
            }
        }
    }
}

