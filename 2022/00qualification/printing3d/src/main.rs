use std::io::{self, BufRead};

const DINK: u32 = 1000000;

fn min3(a: u32, b: u32, c: u32) -> u32 {
    std::cmp::min(std::cmp::min(a, b), c)
}

fn parse_printer(v: &str) -> Vec<u32> {
    let mut s = v.split(' ');
    vec![
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
    ]
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for i in 1..=t {
        let p1 = parse_printer(&lines.next().unwrap().unwrap());
        let p2 = parse_printer(&lines.next().unwrap().unwrap());
        let p3 = parse_printer(&lines.next().unwrap().unwrap());

        print!("Case #{}:", i);
        let inks = vec![
            min3(p1[0], p2[0], p3[0]),
            min3(p1[1], p2[1], p3[1]),
            min3(p1[2], p2[2], p3[2]),
            min3(p1[3], p2[3], p3[3]),
        ];
        if inks.iter().sum::<u32>() >= DINK {
            let mut sum = DINK;
            for val in inks {
                let q = std::cmp::min(val, sum);
                print!(" {}", q);
                sum -= q;
            }
        } else {
            print!(" IMPOSSIBLE");
        }
        println!();
    }
}
