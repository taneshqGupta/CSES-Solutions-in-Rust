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
    let mut inp = 0;
    for _ in 0..9 {
        inp = inp * 10 + cin.next::<usize>();
    }
    let swappable = [
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
    let base: usize = 123456789;
    let mut q = VecDeque::new();
    let mut used = HashSet::new();
    q.push_back((base, 0usize));
    while let Some((momo, d)) = q.pop_front() {
        if inp == momo {
            println!("{}", d);
            break;
        }
        let mut newx = momo;
        let mut xz = [0u8; 9];
        for i in (0..9).rev() {
            xz[i] = (newx % 10) as u8;
            newx /= 10;
        }
        for &(i, j) in &swappable {
            let mut new_xz = xz;
            new_xz.swap(i, j);
            newx = 0;
            for &digit in &new_xz {
                newx = newx * 10 + digit as usize;
            }
            if !used.contains(&newx) {
                q.push_back((newx, d + 1));
                used.insert(newx);
            }
        }
    }
}
