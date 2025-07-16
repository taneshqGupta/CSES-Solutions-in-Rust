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
