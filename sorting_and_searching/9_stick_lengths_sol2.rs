// CSES Sorting & Searching Q-9 :: Stick Lengths
// DateSolved: 13 April 2026
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
    let mut a: Vec<usize> = (0..n).map(|_| cin.next()).collect();

    // property to remember: 
    // median is the required constant
    // let us use inbuilt median-of-medians


    let median;
    if n % 2 == 0 { 
        median = n / 2 - 1;
    } else {
        median = (n - 1) / 2;
    }

    let (_, &mut constant, _) = a.select_nth_unstable(median);
    let mut cost = 0;
    for i in a {
        cost += i.abs_diff(constant);
    }

    println!("{}", cost);
}