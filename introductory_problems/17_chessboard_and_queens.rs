// CSES Introductory-Problems Q-17 :: Chessboard & Queens
// DateSolved: 5 Oct 2025
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
    let inp: Vec<Vec<char>> = (0..8)
        .map(|_| cin.next::<String>().chars().collect())
        .collect();
    fn dfs(
        row: usize,
        ways: &mut usize,
        cols_taken: &mut [bool; 8],
        diag1_taken: &mut [bool; 15],
        diag2_taken: &mut [bool; 15],
        inp: &Vec<Vec<char>>,
    ) {
        if row == 8 {
            *ways += 1;
            return;
        }

        for col in 0..8 {
            if inp[row][col] == '*'
                || cols_taken[col]
                || diag1_taken[row + col]
                || diag2_taken[7 + row - col]
            {
                continue;
            } else {
                cols_taken[col] = true;
                diag1_taken[row + col] = true;
                diag2_taken[7 + row - col] = true;
                dfs(row + 1, ways, cols_taken, diag1_taken, diag2_taken, inp);
                cols_taken[col] = false;
                diag1_taken[row + col] = false;
                diag2_taken[7 + row - col] = false;
            }
        }
    }
    let mut ways = 0;
    let mut cols_taken = [false; 8];
    let mut diag1_taken = [false; 15];
    let mut diag2_taken = [false; 15];
    dfs(
        0,
        &mut ways,
        &mut cols_taken,
        &mut diag1_taken,
        &mut diag2_taken,
        &inp,
    );
    println!("{}", ways);
}
