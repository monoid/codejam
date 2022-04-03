use std::{
    io::{self, BufRead},
    str::FromStr,
};

struct Node {
    max_weight: u32,
    // (max_weight, idx)
    children: Vec<usize>,
}

impl Node {
    fn new(weight: u32) -> Self {
        Self {
            max_weight: weight,
            children: vec![],
        }
    }

    fn insert(&mut self, idx: usize) {
        self.children.push(idx);
    }

    fn calc_max(nodes: &mut [Self], pos: usize) -> u32 {
        let mut mw = nodes[pos].max_weight;
        for chid in 0..nodes[pos].children.len() {
            Self::calc_max(nodes, nodes[pos].children[chid]);
        }
        if let Some(min_child_pos) = nodes[pos]
            .children
            .iter()
            .cloned()
            .min_by_key(|&b| nodes[b].max_weight)
        {
            let min_child = &mut nodes[min_child_pos];
            mw = std::cmp::max(mw, min_child.max_weight);
            min_child.max_weight = 0;
        }
        nodes[pos].max_weight = mw;
        mw
    }

    fn find_sum(nodes: &[Self], pos: usize, sum: &mut u64) {
        *sum += nodes[pos].max_weight as u64;
        for id in &nodes[pos].children {
            Self::find_sum(nodes, *id, sum);
        }
    }
}

fn parse_vec<T: FromStr + Default>(size: usize, line: &str) -> Vec<T>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut v = Vec::with_capacity(size + 1);
    v.push(Default::default());
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
        let fs = parse_vec::<u32>(n, &lines.next().unwrap().unwrap());
        let ps = parse_vec::<usize>(n, &lines.next().unwrap().unwrap());
        assert_eq!(n + 1, fs.len());
        assert_eq!(n + 1, ps.len());

        println!("Case #{}: {}", i, find_sum(fs, ps));
    }
}
