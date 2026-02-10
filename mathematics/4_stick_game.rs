// CSES Mathematics Q-4 :: Stick Game
// DateSolved: 11 Feb 2026
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
    let [n, k] = array::from_fn(|_| cin.next::<usize>());
    let moves: Vec<usize> = (0..k).map(|_| cin.next()).collect();
    let mut win_array: Vec<bool> = vec![false; n + 1];
    'outer: for i in 1..=n {
        for &movex in &moves {
            if movex <= i {
                let win = win_array[i - movex];
                if !win {
                    win_array[i] = true;
                    continue 'outer;
                }
            }
        }
        win_array[i] = false;
    }
    for i in 1..=n {
        print!("{}", if win_array[i] { "W" } else { "L" });
    }
    println!();
}
