// CSES Introductory-Problems Q-21 :: Grid Colouring I
// DateSolved: 6 April 2026
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
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let mut grid: Vec<Vec<char>> = (0..n).map(|_| cin.next::<String>().chars().collect()).collect();
    for i in 0..n {
        for j in 0..m {
            if (i + j) % 2 == 0 {
                grid[i][j] = match grid[i][j] {
                    'A' => 'C',
                    'C' => 'A',
                    _ => 'A',
                }
            } else {
                grid[i][j] = match grid[i][j] {
                    'B' => 'D',
                    'D' => 'B',
                    _ => 'B',
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
