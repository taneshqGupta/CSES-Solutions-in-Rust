// CSES ProblemSet Q-8 :: Two Sets
// DateSolved: 14 Jul 2025
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
    let n: usize = cin.next();
    if n % 4 == 1 || n % 4 == 2 {
        // if n = 4k + 1 or n = 4k + 2,
        // then sum upto n is odd.
        println!("{}", "NO");
        return;
    }
    if n % 4 == 0 {
        // the algo is simple, push the first and the last element into the first set.
        // then push the second and the second last element into the second set.
        // and so on
        println!("{}", "YES");
        println!("{}", n / 2);
        for i in 1..=(n / 4) {
            print!("{} {} ", i, n - i + 1);
        }
        println!();
        println!("{}", n / 2);
        for i in (n / 4 + 1)..=(n / 2) {
            print!("{} {} ", i, n - i + 1);
        }
        println!();
    }
    if n % 4 == 3 {
        // the algo is simple, push the first two elements into one set
        // then push the next two elements into the second set
        // then the next two elements into the first set
        // and so on and so forth.
        // then the last element goes to the second set.
        println!("{}", "YES");
        println!("{}", (n - 3) / 2 + 2);
        for i in (1..(n - 1)).step_by(4) {
            print!("{} {} ", i, i + 1);
        }
        println!();
        println!("{}", (n - 3) / 2 + 1);
        for i in (3..(n - 3)).step_by(4) {
            print!("{} {} ", i, i + 1);
        }
        println!("{}", n);
    }
}
