// CSES ProblemSet Q-7 :: Two Knights
// DateSolved: 9 Jul 2025
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

    for k in 1..=n {
        let ans = match k {
            1 => 0,
            2 => 6,  // 4C2 (4 choose 2)
            3 => 28, //  ( 6*4 + 6*4 + 8*1 ) / 2
            _ => {
                // all the chess grids with k >= 4 can be
                // into seven types of elements
                // namely x1, x2, x3. x4 (corner four elements) and
                // e1, e2 (inner edges) and
                // c (center elements)

                // as usual, divide by two as every pair would be counted twice.

                let (x1, x2, x3, x4) = (4, 4, 4, 4);
                let (e1, e2) = (4 * (k - 4), 4 * (k - 4));
                let c = (k - 4).pow(2);
                let ksq = k.pow(2);
                let mut cont = 0;
                cont += c * (ksq - 9);
                cont += e1 * (ksq - 7);
                cont += e2 * (ksq - 5);
                cont += x1 * (ksq - 5);
                cont += x2 * (ksq - 3);
                cont += x3 * (ksq - 4);
                cont += x4 * (ksq - 4);

                cont / 2
            }
        };
        println!("{}", ans);
    }
}
