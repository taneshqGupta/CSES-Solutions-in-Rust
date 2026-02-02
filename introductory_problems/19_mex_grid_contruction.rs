// CSES Introductory-Problems Q-19 :: Mex Grid Construction
// DateSolved: 1 Feb 2026
// SolvedBy: taneshqGupta

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
    let n: usize = cin.next();
    for r in 0..n {
        for c in 0..n {
            print!("{} ", r ^ c);
        }
        println!();
    }
}
