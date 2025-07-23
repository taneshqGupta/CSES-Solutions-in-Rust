// CSES ProblemSet Q-14 :: Tower of Hanoi
// DateSolved: 18 Jul 2025
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

#[derive(Default)]
struct Tower(Vec<(u8, u8)>);
impl Tower {
    fn build(&mut self, n: u8, from: u8, aux: u8, to: u8) {
        if n > 0 {
            self.build(n - 1, from, to, aux);
            self.0.push((from, to));
            self.build(n - 1, aux, from, to);
        }
    }
}

fn main() {
    let mut cin = Scanner::default();
    let mut tower = Tower::default();
    tower.build(cin.next(), 1, 2, 3);
    println!("{}", tower.0.len());
    for (from, to) in tower.0 {
        println!("{} {}", from, to);
    }
}
