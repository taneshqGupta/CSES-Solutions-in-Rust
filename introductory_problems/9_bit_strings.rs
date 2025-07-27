// CSES ProblemSet Q-9 :: Bit Strings
// DateSolved: 15 Jul 2025
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
    let mut n: u32 = cin.next();

    // to compute: 2^n % (1e9 + 7)

    // --- Linear Exponentiation Solution --- O(N)
    // let mut ans = 2;

    // for _ in 1..n {
    //     ans = (ans * 2) % (1e9 as usize + 7);
    // }

    // println!("{}", ans);
    // --- Linear Exponentiation Solution ---

    // --- Binary Exponentiation Solution --- O(log(N))
    let mut ans = 1;
    let mut base = 2;
    const MOD: usize = 1e9 as usize + 7;
    
    while n > 0 {
        if n % 2 == 1 {
            ans = (ans * base) % MOD;
        }
        base = (base * base) % MOD;
        n /= 2;
    }

    println!("{}", ans);
    // --- Binary Exponentiation Solution
}
