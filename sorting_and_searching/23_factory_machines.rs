// CSES Sorting & Searching Q-23 :: Factory Machines
// DateSolved: 25 April 2026
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
    /*
        If we pick an arbitrary time T,
        the number of products the i-th machine can produce in that time is exactly floor(T / ki).

        The total number of products produced
        by all machines in time T is the sum of these individual amounts.
        This total is a monotonically increasing function of T.
        Because the function is monotonic,
        we do not need to check every single time value sequentially.
        We can binary search the answer over the range of possible times
        to find the minimum T where the total products reach or exceed t.
    */
    let n: usize = cin.next();
    let t: usize = cin.next();
    let machines: Vec<usize> = (0..n).map(|_| cin.next()).collect();

    let mut low = 1;
    let mut high = 1e18 as usize + 1;

    while low != high {
        let mid = low + (high - low) / 2;
        let mut products = 0;
        for &k in &machines {
            products += mid / k;
            if products >= t {
                break;
            }
        }
        if products >= t {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    println!("{}", low);
}