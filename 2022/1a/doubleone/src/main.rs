use std::io::{self, BufRead};

fn doubleone(line: &str) -> String {
    let mut res = String::new();
    let line = line.as_bytes();
    // I would use u8::MAX, but the Google's Rust compiler is too old.
    let mut prev_val = 255;
    let mut prev_cnt = 0usize;

    for &b in line {
        match b.cmp(&prev_val) {
            std::cmp::Ordering::Less => {
                prev_val = b;
                prev_cnt = 1;
            }
            std::cmp::Ordering::Equal => {
                prev_cnt += 1;
            }
            std::cmp::Ordering::Greater => {
                for _ in 0..prev_cnt {
                    res.push(prev_val as char);
                }
                prev_val = b;
                prev_cnt = 1;
            }
        }
        res.push(b as char);
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        println!("Case #{}: {}", i, doubleone(line.trim()));
    }
}
