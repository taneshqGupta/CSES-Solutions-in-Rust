// CSES Introductory-Problems Q-23 :: String Reorder
// DateSolved: 8 April 2026
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
    let s: String = cin.next();
    let n = s.len();
    let mut f: [usize; 26] = [0; 26];
    for c in s.bytes() {
        f[(c - b'A') as usize] += 1;
    }
    // dbg!(&f);

    /* Let us simply solve in O(n * 26 * 26) */

    let mut result: Vec<u8> = Vec::with_capacity(n);

    let mut last_char = None;
    for i in 0..n {
        let remaining_length = n - 1 - i;
        let mut found = false;
        for c in 0..26 {
            if f[c] > 0 && Some(c) != last_char {
                f[c] -= 1;

                let f_max: usize = *f.iter().max().unwrap();
                if f_max <= remaining_length.div_ceil(2) {
                    found = true;
                    last_char = Some(c);
                    result.push(c as u8);
                    break;
                }

                f[c] += 1;
            }
        }
        if !found {
            println!("-1");
            return;
        }
    }
    for c in result {
        print!("{}", (c + b'A') as char);
    }
}