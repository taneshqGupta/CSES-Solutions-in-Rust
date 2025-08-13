// CSES Interactive-Problems Q-3 :: K'th Highest Score
// DateSolved: 2 Aug 2025
// SolvedBy: taneshqGupta 

std::{array, collections::HashMap};

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

fn query(
    cin: &mut Scanner,
    n: usize,
    finland: bool,
    hashx: &mut HashMap<(bool, usize), usize>,
) -> usize {
    if n == 0 {
        return 1e9 as usize + 1;
    }
    if !hashx.contains_key(&(finland, n)) {
        println!("{} {}", if finland { 'F' } else { 'S' }, n);
        hashx.insert((finland, n), cin.next());
    }
    return *hashx.get(&(finland, n)).unwrap();
}

fn binary(
    cin: &mut Scanner,
    k: usize,
    mut low: usize,
    mut high: usize,
    finland: bool,
    hashx: &mut HashMap<(bool, usize), usize>,
) -> usize {
    let mut result = 0;
    while low <= high {
        // dbg!(low, high, result);
        let mid = low + (high - low) / 2;
        let this = query(cin, mid, finland, hashx);
        let that = query(cin, k - mid, !finland, hashx);
        if this < that {
            result = this;
            high = mid - 1;
        } else {
            low = mid + 1;
        }
        // dbg!(low, high, result);
    }
    return result;
}

fn main() {
    let mut cin = Scanner::default();
    let [n, k] = array::from_fn(|_| cin.next::<usize>());
    let low;
    let high;
    if k <= n {
        low = 0;
        high = k;
    } else {
        low = k - n;
        high = n;
    }
    let mut hashx = HashMap::new();
    let fin_ans = binary(&mut cin, k, low, high, true, &mut hashx);
    let swe_ans = binary(&mut cin, k, low, high, false, &mut hashx);
    println!("! {}", fin_ans.max(swe_ans));
}