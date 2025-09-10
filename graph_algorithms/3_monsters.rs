// CSES Graph-Algorithms Q-3 :: Monsters
// DateSolved: 11 Sep 2025
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

// fn debug_moves(a: &Vec<Vec<char>>) {
//     for vecx in a {
//         for aa in vecx {
//             let _ = write!(stderr(), "{}", aa);
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }
// fn debug_usize(a: &Vec<Vec<usize>>) {
//     for vecx in a {
//         for aa in vecx {
//             if *aa > (1e15 as usize) {
//                 let _ = write!(stderr(), " - ");
//             } else {
//                 let _ = write!(stderr(), " {} ", aa);
//             }
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }
// fn debug_bool(a: &Vec<Vec<bool>>) {
//     for vecx in a {
//         for &aa in vecx {
//             let _ = write!(stderr(), "{}", if aa { 1 } else { 0 });
//         }
//         let _ = writeln!(stderr());
//     }
//     let _ = writeln!(stderr());
// }

fn main() {
    let mut cin = Scanner::default();
    let [n, m] = array::from_fn(|_| cin.next::<usize>());
    let map: Vec<Vec<char>> = (0..n)
        .map(|_| cin.next::<String>().chars().collect())
        .collect();
    // debug_moves(&map);
    let mut monsters = vec![];
    let mut src = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'M' {
                monsters.push((i, j));
            }
            if map[i][j] == 'A' {
                src = (i, j);
            }
        }
    }
    // dbg!(&monsters);
    let mut d_mon = vec![vec![1e16 as usize; m]; n];
    let mut q = VecDeque::new();
    let mut used = vec![vec![false; m]; n];
    for &(x, y) in &monsters {
        d_mon[x][y] = 0;
        q.push_back((x, y));
        used[x][y] = true;
    }
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        let mut moves = Vec::new();

        if momo.0 > 0 {
            moves.push((momo.0 - 1, momo.1));
        }
        if momo.1 > 0 {
            moves.push((momo.0, momo.1 - 1));
        }
        if momo.0 < (n - 1) {
            moves.push((momo.0 + 1, momo.1));
        }
        if momo.1 < (m - 1) {
            moves.push((momo.0, momo.1 + 1));
        }

        for (x, y) in moves {
            if !used[x][y] && map[x][y] != '#' && map[x][y] != 'M' {
                used[x][y] = true;
                q.push_back((x, y));
                d_mon[x][y] = d_mon[x][y].min(d_mon[momo.0][momo.1] + 1);
            }
        }
        // debug_bool(&used);
        // debug_usize(&d_mon);
        // dbg!(&q, momo);
        // let _ = writeln!(stderr(), "\n\n");
    }
    // debug_usize(&d_mon);

    let mut q = VecDeque::new();
    let mut used = vec![vec![false; m]; n];
    let mut last_move = vec![vec!['K'; m]; n];
    let mut parent = vec![vec![src; m]; n];
    let mut d_src = vec![vec![0; m]; n];

    q.push_back(src);
    used[src.0][src.1] = true;

    let mut ys = false;
    let mut end = (0, 0);

    // dbg!(&q);
    while !q.is_empty() {
        let momo = q.pop_front().unwrap();
        if momo.0 == 0 || momo.0 == (n - 1) || momo.1 == 0 || momo.1 == (m - 1) {
            ys = true;
            end = momo;
            break;
        }
        let mut moves = Vec::new();

        if momo.0 > 0 {
            moves.push((momo.0 - 1, momo.1));
        }
        if momo.1 > 0 {
            moves.push((momo.0, momo.1 - 1));
        }
        if momo.0 < (n - 1) {
            moves.push((momo.0 + 1, momo.1));
        }
        if momo.1 < (m - 1) {
            moves.push((momo.0, momo.1 + 1));
        }

        for (x, y) in moves {
            // dbg!(x, y, d_mon[x][y], d_src[momo.0][momo.1]);
            if !used[x][y] && map[x][y] != '#' && d_mon[x][y] > (d_src[momo.0][momo.1] + 1) {
                used[x][y] = true;
                q.push_back((x, y));
                d_src[x][y] = d_src[momo.0][momo.1] + 1;
                parent[x][y] = momo;
                if x < momo.0 {
                    last_move[x][y] = 'U';
                }
                if y < momo.1 {
                    last_move[x][y] = 'L';
                }
                if x > momo.0 {
                    last_move[x][y] = 'D';
                }
                if y > momo.1 {
                    last_move[x][y] = 'R';
                }
            }
        }
        // debug_bool(&used);
        // debug_usize(&d_src);
        // dbg!(&q, momo);
        // let _ = writeln!(stderr(), "\n\n");
    }
    // debug_moves(&last_move);
    if !ys {
        println!("NO");
    } else {
        println!("YES");
        println!("{}", d_src[end.0][end.1]);
        let mut moves = Vec::new();
        let mut curr = end;
        while curr != src {
            moves.push(last_move[curr.0][curr.1]);
            curr = parent[curr.0][curr.1];
        }
        moves.reverse();
        for c in moves {
            print!("{}", c);
        }
        println!();
    }
}
