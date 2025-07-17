// CSES ProblemSet Q-11 :: Coin Piles
// DateSolved: 17 Jul 2025
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
    let t: usize = cin.next();
    for _ in 0..t {
        let (a, b): (usize, usize) = (cin.next(), cin.next());
        if (a + b) % 3 != 0 {
            println!("{}", "NO");
            continue;
        }
        let steps = (a + b) / 3;
        if std::cmp::min(a, b) < steps {
            println!("{}", "NO");
            continue;
        }
        println!("{}", "YES");
    }
}
