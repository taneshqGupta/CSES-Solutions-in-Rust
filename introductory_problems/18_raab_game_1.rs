// CSES Introductory-Problems Q-18 :: Raab Game I
// DateSolved: 1 Feb 2026
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
    for _ in 0..(cin.next()) {
        let [n, a, b] = array::from_fn(|_| cin.next::<usize>());
        if (a + b > n) || (a > 0 && b == 0) || (a == 0 && b > 0) {
            println!("NO");
            continue;
        }
        let mut x = Vec::new();
        let mut y = Vec::new();
        let draws = n - a - b;
        for i in 1..=draws {
            x.push(i);
            y.push(i);
        }
        let mut momo = Vec::new();
        for i in 0..a {
            x.push(n - i);
            momo.push(draws+1+i);
        }
        momo.reverse();
        y.append(&mut momo);
        let mut momo = Vec::new();
        for i in 0..b {
            y.push(n - i);
            momo.push(draws+1+i);
        }
        momo.reverse();
        x.append(&mut momo);
        println!("YES");
        for c in x {
            print!("{} ", c);
        }
        println!();
        for c in y {
            print!("{} ", c);
        }
        println!();
    }
}
