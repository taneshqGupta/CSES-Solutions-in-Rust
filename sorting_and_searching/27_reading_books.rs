// CSES Sorting & Searching Q-27 :: Reading Books
// DateSolved: 27 April 2026
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
    let t: Vec<usize> = (0..n).map(|_| cin.next()).collect();
    /* 
        Case 1: 
        The longest book takes strictly more time than all other books combined. 
        If one person starts with the longest book and the other starts with the rest, 
        the second person will finish all the smaller books and be forced to sit idle. 
        They cannot start the longest book until the first person finishes it. 
        The total time is completely bottlenecked by this single massive book.

        Case 2: 
        The longest book takes less than or equal to the 
        total time of all other books combined. In this scenario, 
        it is always possible to arrange the reading schedules 
        so that neither person ever has to wait for the other. 
        Since both readers can be kept busy 100% of the time, 
        the minimum time required is simply the time it takes 
        to read the entire pile of books without any breaks.
        This schedule can be constructed by making 
        first person read the books in any arbitrary order,
        and then making the second person follow the same order,
        but after (S / 2) time units wrapping around end to 0, 
        where S is the total time of all books combined.
        if longest book is of time M. then M <= S - M.
        or M <= S / 2. That means, no book is longer than S / 2 time.
        Thus, no conflict can occur in this schedule.
        
    */
    let longest_book = *t.iter().max().unwrap();
    let mut found = false;
    let mut rest_sum = 0;
    for i in 0..n {
        if !found && t[i] == longest_book {
            found = true;
            continue;
        }
        rest_sum += t[i];
    }
    if longest_book > rest_sum {
        println!("{}", 2 * longest_book);
    } else {
        println!("{}", longest_book + rest_sum);
    }
}