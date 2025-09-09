// CSES Graph-Algorithms Q-1 :: Labyrinths
// DateSolved: 9 Sep 2025
// SolvedBy: taneshqGupta

use std::{
    array,
    collections::VecDeque,
    // io::{stderr, Write},
};

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

// fn debug_bool(a: &Vec<Vec<bool>>) {
//     for vecx in a {
//         for &aa in vecx {
//             let _ = write!(stderr(), "{}", if aa { 1 } else { 0 });
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }
// fn debug_usize(a: &Vec<Vec<usize>>) {
//     for vecx in a {
//         for aa in vecx {
//             let _ = write!(stderr(), "{}", aa);
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }
// fn debug_moves(a: &Vec<Vec<char>>) {
//     for vecx in a {
//         for aa in vecx {
//             let _ = write!(stderr(), "{}", aa);
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }
// fn debug_parents(a: &Vec<Vec<(usize, usize)>>) {
//     for vecx in a {
//         for aa in vecx {
//             let _ = write!(stderr(), "({},{})", aa.0, aa.1);
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }

fn main() {
    let mut cin = Scanner::default();
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let mut a = vec![vec![' '; m]; n];
    let mut source = (0, 0);
    let mut end = (0, 0);
    for i in 0..n {
        let s = cin.next::<String>();
        a[i] = s.chars().collect();
        for (j, c) in s.chars().enumerate() {
            if c == 'A' {
                source = (i, j);
            }
            if c == 'B' {
                end = (i, j);
            }
        }
    }
    // dbg!(source, end);
    let mut parents = vec![vec![source; m]; n];
    let mut last_move = vec![vec!['F'; m]; n];
    let mut distances = vec![vec![0; m]; n];
    let mut used = vec![vec![false; m]; n];
    let mut q = VecDeque::new();
    q.push_back(source);
    used[source.0][source.1] = true;
    let mut ys = false;
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        // dbg!(&momo, &q);
        // debug_bool(&used);
        // debug_usize(&distances);
        // debug_parents(&parents);
        // let _ = writeln!(stderr());
        if momo == end {
            ys = true;
            break;
        }
        let d = distances[momo.0][momo.1];
        if momo.0 > 0 {
            if !used[momo.0 - 1][momo.1] {
                if a[momo.0 - 1][momo.1] != '#' {
                    used[momo.0 - 1][momo.1] = true;
                    q.push_back((momo.0 - 1, momo.1));
                    parents[momo.0 - 1][momo.1] = momo;
                    last_move[momo.0 - 1][momo.1] = 'U';
                    distances[momo.0 - 1][momo.1] = d + 1;
                }
            }
        }
        if momo.1 > 0 {
            if !used[momo.0][momo.1 - 1] {
                if a[momo.0][momo.1 - 1] != '#' {
                    used[momo.0][momo.1 - 1] = true;
                    q.push_back((momo.0, momo.1 - 1));
                    parents[momo.0][momo.1 - 1] = momo;
                    last_move[momo.0][momo.1 - 1] = 'L';
                    distances[momo.0][momo.1 - 1] = d + 1;
                }
            }
        }
        if momo.0 < (n - 1) {
            if !used[momo.0 + 1][momo.1] {
                if a[momo.0 + 1][momo.1] != '#' {
                    used[momo.0 + 1][momo.1] = true;
                    q.push_back((momo.0 + 1, momo.1));
                    parents[momo.0 + 1][momo.1] = momo;
                    last_move[momo.0 + 1][momo.1] = 'D';
                    distances[momo.0 + 1][momo.1] = d + 1;
                }
            }
        }
        if momo.1 < (m - 1) {
            if !used[momo.0][momo.1 + 1] {
                if a[momo.0][momo.1 + 1] != '#' {
                    used[momo.0][momo.1 + 1] = true;
                    q.push_back((momo.0, momo.1 + 1));
                    parents[momo.0][momo.1 + 1] = momo;
                    last_move[momo.0][momo.1 + 1] = 'R';
                    distances[momo.0][momo.1 + 1] = d + 1;
                }
            }
        }
        // dbg!(&momo, &q);
        // debug_bool(&used);
        // debug_usize(&distances);
        // debug_parents(&parents);
        // let _ = writeln!(stderr());
    }
    // debug_moves(&last_move);
    if !ys {
        println!("NO");
    } else {
        println!("YES");
        println!("{}", distances[end.0][end.1]);
        let mut moves = Vec::new();
        let mut curr = end;
        while curr != source {
            moves.push(last_move[curr.0][curr.1]);
            curr = parents[curr.0][curr.1];
        }
        moves.reverse();
        for c in moves {
            print!("{}", c);
        }
        println!();
    }
}