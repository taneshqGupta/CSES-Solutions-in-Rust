// CSES Introductory-Problems Q-11 :: Coin Piles
// DateSolved: 17 Jul 2025
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
    let t = cin.next();
    for _ in 0..t {
        let [a, b] = array::from_fn(|_| cin.next::<usize>());
        if (a + b) % 3 != 0 {
            println!("NO");
            continue;
        }
        let steps = (a + b) / 3;
        if a.min(b) < steps {
            println!("NO");
            continue;
        }
        println!("YES");
    }
}
