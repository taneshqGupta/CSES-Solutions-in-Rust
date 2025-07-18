// CSES ProblemSet Q-13 :: Gray Code
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

fn gray(n: u8) -> Vec<Vec<bool>> {
    if n == 1 {
        return vec![vec![false], vec![true]];
    }
    let aa = gray(n - 1);
    aa.iter()
        .map(|x| [&x[..], &[false]].concat())
        .chain(aa.iter().rev().map(|x| [&x[..], &[true]].concat()))
        .collect()
}

fn main() {
    let mut cin = Scanner::new();
    let aa = gray(cin.next());
    for a in aa {
        for &c in a.iter().rev() {
            print!("{}", if c { 1 } else { 0 });
        }
        println!();
    }
}
