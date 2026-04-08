// CSES Sorting & Searching Q-3 :: Ferris Wheel
// DateSolved: 8 April 2026
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
    let [n, x] = array::from_fn(|_| cin.next::<usize>());
    let mut weights: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    weights.sort_unstable();

    let mut gondolas = 0;

    if n == 1 {
        println!("1");
        return;
    }

    let mut i = 0;
    let mut j = n - 1;
    while i < n && j < n && j >= i {
        while weights[i] + weights[j] > x {
            if j == 0 {
                break;
            }
            j -= 1;
            gondolas += 1;
        }
        if j > i {
            i += 1;
            j -= 1;
            gondolas += 1;
        }
        if j == i {
            gondolas += 1;
            break;
        }
    }

    println!("{}", gondolas);
}
