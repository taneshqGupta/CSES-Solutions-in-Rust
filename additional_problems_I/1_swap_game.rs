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

const POWERS: [isize; 9] = [
    100000000, 10000000, 1000000, 100000, 10000, 1000, 100, 10, 1,
];
fn swap(a: &usize, (i, j): (usize, usize)) -> usize {
    let mut b = *a as isize;
    let i_multiplier = POWERS[i];
    let j_multiplier = POWERS[j];
    let i_digit = (b / i_multiplier) % 10;
    let j_digit = (b / j_multiplier) % 10;
    b -= i_digit * i_multiplier;
    b += j_digit * i_multiplier;
    b -= j_digit * j_multiplier;
    b += i_digit * j_multiplier;
    return b as usize;
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
        for &c in &swappable {
            let newx = swap(&momo, c);
            if !used.contains(&newx) {
                q.push_back((newx, d + 1));
                used.insert(newx);
            }
        }
    }
}
