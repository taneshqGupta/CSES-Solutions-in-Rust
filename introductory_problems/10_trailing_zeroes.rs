// CSES Introductory-Problems Q-10 :: Trailing Zeroes
// DateSolved: 16 Jul 2025
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
    let mut count = 0;
    let mut i = 5;
    while i <= n { // algo to find the number of prime factors of i in n!.
        count += n / i;
        i *= 5;
    }
    println!("{}", count);
}