// CSES ProblemSet Q-9 :: Bit Strings
// DateSolved: 15 Jul 2025
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
    let n: u32 = cin.next();

    let mut ans = 2;

    for _ in 1..n {
        ans = (ans * 2) % (1e9 as usize + 7);
    }

    println!("{}", ans);
}
