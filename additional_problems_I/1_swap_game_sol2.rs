// CSES Additional Problems I - Q-1 :: Swap Game - Sol-2 (Using A*)
// DateSolved: 11 Sep 2025
// SolvedBy: taneshqGupta

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
 
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
 
fn heuristic(a: &usize, mpx: &[usize]) -> usize {
    let b = *a as isize;
    let mut manhattan_distance = 0;
    for i in 0..9 {
        let digit = (b as isize / POWERS[i]) as usize % 10;
        let j = mpx[digit - 1];
 
        let r1 = i as isize / 3;
        let c1 = i as isize % 3;
        let r2 = j as isize / 3;
        let c2 = j as isize % 3;
        manhattan_distance += r1.abs_diff(r2) + c1.abs_diff(c2);
    }
    (manhattan_distance as usize) / 2
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
    let mut mpx = [0; 9];
    for i in 0..9 {
        let digit: usize = cin.next();
        inp = inp * 10 + digit;
        mpx[digit - 1] = i;
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
 
    let mut q = BinaryHeap::new();
    let mut costs = HashMap::new();
 
    let start_d = 0;
    let h = heuristic(&base, &mpx);
    q.push((Reverse(start_d + h), start_d, base));
    costs.insert(base, start_d);
 
    while let Some((_, d, momo)) = q.pop() {
        if inp == momo {
            println!("{}", d);
            return;
        }
 
        if d > *costs.get(&momo).unwrap_or(&usize::MAX) {
            continue;
        }
 
        for &c in &swappable {
            let newx = swap(&momo, c);
 
            let new_d = d + 1;
            if new_d < *costs.get(&newx).unwrap_or(&usize::MAX) {
                costs.insert(newx, new_d);
                let h = heuristic(&newx, &mpx);
                q.push((Reverse(new_d + h), new_d, newx));
            }
        }
    }
}