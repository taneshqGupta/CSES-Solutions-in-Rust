// CSES Additional Problems I - Q-1 :: Swap Game
// DateSolved: 11 Sep 2025
// SolvedBy: taneshqGupta

use std::collections::{HashMap, VecDeque};

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
    let base: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut q = VecDeque::new();
    let mut distances = HashMap::new();
    q.push_back(base.clone());
    distances.insert(base.clone(), 0);
    while let Some(momo) = q.pop_front() {
        for &(i, j) in &swappable {
            let mut newx = momo.clone();
            newx.swap(i, j);
            if !distances.contains_key(&newx) {
                q.push_back(newx.clone());
                distances.insert(newx.clone(), *distances.get(&momo).unwrap() + 1);
            }
        }
    }
    let inputx: Vec<u8> = (0..9).map(|_| cin.next()).collect();
    println!("{}", *distances.get(&inputx).unwrap());
}
