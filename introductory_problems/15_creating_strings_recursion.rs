// CSES Introductory-Problems Q-15 :: Creating Strings Using Recursion
// DateSolved: 2 Feb 2026
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
    let mut freq: [u8; 26] = [0; 26];
    let s = cin.next::<String>();
    for c in s.bytes() {
        freq[(c - b'a') as usize] += 1;
    }
    let mut ans_list: Vec<String> = Vec::new();
    let mut current_string = String::new();
    fn ans(
        freq: &mut [u8; 26],
        ans_list: &mut Vec<String>,
        current_string: &mut String,
        final_len: usize,
    ) {
        if current_string.len() == final_len {
            ans_list.push(current_string.to_string());
            // dbg!(&ans_list);
            return;
        }
        for i in 0..26 {
            // dbg!(i);
            if freq[i] > 0 {
                // dbg!(&freq[i]);
                freq[i] -= 1;
                current_string.push((b'a' + i as u8) as char);
                // dbg!(&current_string);
                ans(freq, ans_list, current_string, final_len);
                current_string.pop();
                freq[i] += 1;
            }
        }
    }
    ans(&mut freq, &mut ans_list, &mut current_string, s.len());
    println!("{}", ans_list.len());
    for s in ans_list {
        println!("{}", s);
    }
}
