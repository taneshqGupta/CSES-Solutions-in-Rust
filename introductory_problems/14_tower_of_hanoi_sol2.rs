// CSES Introductory-Problems Q-14 :: Tower of Hanoi
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
 
fn tower(n: u8, a: char, b: char, c: char, s: &mut String) {
    if n > 0 {
        tower(n - 1, a, c, b, s);
        s.push(a);
        s.push(' ');
        s.push(c);
        s.push('\n');
        tower(n - 1, b, a, c, s);
    }
}
 
fn main() {
    let mut cin = Scanner::default();
    let n = cin.next();
    print!("{}\n", (1 << n) - 1);
    let mut s = String::new();
    tower(n, '1', '2', '3', &mut s);
    print!("{}", s);
}