use std::{
    io::{self, BufRead},
    str::FromStr,
};

struct Node {
    max_weight: u32,
    // (max_weight, idx)
    children: Vec<(u32, usize)>,
}

impl Node {
    fn new(weight: u32) -> Self {
        Self {
            max_weight: weight,
            children: vec![],
        }
    }

    fn insert(&mut self, idx: usize) {
        self.children.push((0, idx));
    }

    fn calc_max(nodes: &mut [Self], pos: usize) -> u32 {
        let mut mw = nodes[pos].max_weight;
        for chid in 0..nodes[pos].children.len() {
            let ch_max_weight = Self::calc_max(nodes, nodes[pos].children[chid].1);
            nodes[pos].children[chid].0 = ch_max_weight;
        }
        nodes[pos].children.shrink_to_fit();
        nodes[pos].children.sort_unstable();
        if let Some(&(ch_w, p)) = nodes[pos].children.first() {
            mw = std::cmp::max(mw, ch_w);
            nodes[p].max_weight = 0;
        }
        nodes[pos].max_weight = mw;
        mw
    }

    fn find_sum(nodes: &[Self], pos: usize, sum: &mut u64) {
        *sum += nodes[pos].max_weight as u64;
        for (_, id) in &nodes[pos].children {
            Self::find_sum(nodes, *id, sum);
        }
    }
}

fn parse_vec<T: FromStr + Default>(line: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut v = vec![Default::default()];
    v.extend(line.split(' ').map(|p| p.parse().unwrap()));
    v
}

fn find_sum(fs: Vec<u32>, ps: Vec<usize>) -> u64 {
    let mut nodes: Vec<_> = fs.into_iter().map(Node::new).collect();
    for (s, &to) in ps.iter().enumerate().skip(1) {
        nodes[to].insert(s);
    }
    Node::calc_max(&mut nodes, 0);
    let mut sum = 0;
    Node::find_sum(&nodes, 0, &mut sum);
    sum
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for i in 1..=t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let fs = parse_vec::<u32>(&lines.next().unwrap().unwrap());
        let ps = parse_vec::<usize>(&lines.next().unwrap().unwrap());
        assert_eq!(n + 1, fs.len());
        assert_eq!(n + 1, ps.len());

        println!("Case #{}: {}", i, find_sum(fs, ps));
    }
}
