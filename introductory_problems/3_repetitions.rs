// CSES Introductory-Problems Q-3 :: Repetitions
// DateSolved: 24 Jun 2025
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
    
    const N: usize = 2e6 as usize;
    let mut aa: [u32; N] = [0; N];
    let s: String = cin.next();
    let n: usize = s.len();
    let ss: Vec<char> = s.chars().collect();

    for i in 1..n {
        if ss[i] == ss[i-1] {
            aa[i] = aa[i-1] + 1;
        } 
    } 

    let ans: u32 = *aa.iter().max().unwrap();
    println!("{}", ans + 1);
}
