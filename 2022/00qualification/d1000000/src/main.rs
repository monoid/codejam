use std::io::{self, BufRead};

fn solve(mut dices: Vec<u32>) -> u32 {
    let mut r = 0;
    dices.sort_unstable();
    for m in dices {
        if m > r {
            r += 1;
        }
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock().lines();

    let t: usize = stdin.next().unwrap().unwrap().parse().unwrap();
    for i in 1..=t {
        let d: usize = stdin.next().unwrap().unwrap().parse().unwrap();
        let dices_str = stdin.next().unwrap().unwrap();
        let dices: Vec<u32> = dices_str.split(' ').map(|l| l.parse().unwrap()).collect();
        std::mem::drop(dices_str);
        assert_eq!(d, dices.len());
        println!("Case #{}: {}", i, solve(dices));
    }
}
