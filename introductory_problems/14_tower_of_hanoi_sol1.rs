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

struct Tower(Vec<(u8, u8)>);
impl Tower {
    fn new() -> Self {
        Tower(Vec::new())
    }
    fn build(&mut self, n: u8, from: u8, aux: u8, to: u8) {
        if n > 0 {
            self.build(n - 1, from, to, aux);
            self.0.push((from, to));
            self.build(n - 1, aux, from, to);
        }
    }
}

fn main() {
    let mut cin = Scanner::new();
    let mut tower = Tower::new();
    tower.build(cin.next(), 1, 2, 3);
    println!("{}", tower.0.len());
    for (from, to) in tower.0 {
        println!("{} {}", from, to);
    }
}
