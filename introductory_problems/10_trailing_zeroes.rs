// CSES ProblemSet Q-10 :: Trailing Zeroes
// DateSolved: 16 Jul 2025
// SolvedBy: taneshqGupta

struct Scanner(Vec<String>);
impl Scanner {
    fn new() -> Self {
        let input = std::io::read_to_string(std::io::stdin()).unwrap();
        Scanner(input.split_whitespace().map(String::from).rev().collect())
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.0.pop().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut cin = Scanner::new();
    let n: usize = cin.next();
    let mut count = 0;
    let mut i = 5;
    while i <= n { // algo to find the number of prime factors of i in n.
        count += n / i;
        i *= 5;
    }
    println!("{}", count);
}
