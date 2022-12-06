//{"name":"oprava_silnic","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"oprava_silnic"}}}

use std::cmp::Ordering;
use std::time::Instant;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::collections::disjoint_set::DisjointSet;


#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Edge {
    cost: u64,
    v1: usize,
    v2: usize,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let k: usize = input.read();

    let mut edges = vec![];
    for _ in 0..m {
        edges.push(Edge {
            v1: input.read::<usize>()-1,
            v2: input.read::<usize>()-1,
            cost: input.read(),
        });
    }


    let mut healthy = vec![true; n];

    for _ in 0..k {
        healthy[input.read::<usize>()-1] = false;
    }

    edges.sort();

    if edges.len() == 0 {
        out_line!(-1);
        return;
    }

    if k==n {
        if n==2 {
            out_line!(edges[0].cost);
            return;
        }
        else {
            out_line!(-1);
            return;
        }
    }

    let mut healthy_edges = vec![];
    for edge in &edges {
        if healthy[edge.v1] && healthy[edge.v2] {
            healthy_edges.push(edge.clone());
        }
    }

    let mut connected = 1;

    let mut answer = 0;
    let mut ds = DisjointSet::new(n);
    for edge in healthy_edges {
        if ds.join(edge.v1, edge.v2) {
            answer += edge.cost;
            connected += 1;
        }
    }

    if connected != n-k {
        out_line!(-1);
        return;
    }


    let mut fixed = healthy.clone();

    for edge in edges {
        if (healthy[edge.v1] ^ healthy[edge.v2]) && (!fixed[edge.v1] || !fixed[edge.v2]) {
            fixed[edge.v1] = true;
            fixed[edge.v2] = true;
            answer += edge.cost;
        }
    }

    for i in 0..n {
        if !fixed[i] {
            out_line!(-1);
            return;
        }
    }

    out_line!(answer);

}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let now = Instant::now();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
