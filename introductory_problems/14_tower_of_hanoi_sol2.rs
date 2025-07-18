// CSES ProblemSet Q-14 :: Tower of Hanoi
// DateSolved: 18 Jul 2025
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

fn tower(n: u8, a: u8, b: u8, c: u8) {
    if n > 0 {
        tower(n - 1, a, c, b);
        println!("{} {}", a, c);
        tower(n - 1, b, a, c);
    }
}

fn main() {
    let mut cin = Scanner::new();
    let n: u8 = cin.next();
    println!("{}", u16::pow(2, n as u32) - 1);
    tower(n, 1, 2, 3);
}