// CSES Introductory-Problems Q-20 :: Knight Moved Grid
// DateSolved: 4 April 2026
// SolvedBy: taneshqGupta

use std::collections::VecDeque;

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

#[derive(Debug, Default)]
struct Position {
    value: usize,
    i: usize,
    j: usize,
}

fn main() {
    let mut cin = Scanner::default();
    let n: usize = cin.next();
    let mut grid = vec![vec![0; n]; n];

    const MOVES: [(isize, isize); 8] = [
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (-1, -2),
        (1, -2),
        (-1, 2),
        (1, 2),
    ];

    let mut q: VecDeque<Position> = VecDeque::new();
    let mut used = vec![vec![false; n]; n];
    q.push_back(Position::default());
    used[0][0] = true;

    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        for &(di, dj) in &MOVES {
            let nextposition = (momo.i as isize + di, momo.j as isize + dj);
            if nextposition.0 >= 0
                && nextposition.0 < n as isize
                && nextposition.1 >= 0
                && nextposition.1 < n as isize
            {
                let nextposition = Position {
                    value: momo.value + 1,
                    i: nextposition.0 as usize,
                    j: nextposition.1 as usize,
                };
                if !used[nextposition.i][nextposition.j] {
                    used[nextposition.i][nextposition.j] = true;
                    grid[nextposition.i][nextposition.j] = nextposition.value;
                    q.push_back(nextposition);
                }
            }
        }
    }

    for row in grid {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }
}