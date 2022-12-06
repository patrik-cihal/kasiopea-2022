//{"name":"vernostni_karticky","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"vernostni_karticky"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Node {
    cost: u64,
    index: usize,
    parent: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_paths(g: &Vec<Vec<usize>>, c: &Vec<u64>) -> Vec<usize> {
    let mut bh = BinaryHeap::new();
    bh.push(Node {
        cost: c[0],
        index: 0,
        parent: 0,
    });
    let mut paths = vec![vec![]; g.len()];
    let mut bc = vec![u64::MAX; g.len()];
    while let Some(node) = bh.pop() {
        if bc[node.index] < node.cost {
            continue;
        }
        for &neighbor in &g[node.index] {
            let new_cost = node.cost + c[neighbor];
            if bc[neighbor] >= new_cost {
                bc[neighbor] = new_cost;
                bh.push(Node {
                    cost: new_cost,
                    index: neighbor,
                    parent: node.index,
                });
            }
        }
        paths[node.index].push(node.parent);
    }

    let mut nodes_way = vec![g.len() - 1];
    let mut result = vec![];
    let mut visited = vec![false; g.len()];
    while !nodes_way.is_empty() {
        let mut new_nodes_way = vec![];
        for &node_way in &nodes_way {
            if visited[node_way] {
                continue;
            }
            visited[node_way] = true;
            if paths[node_way][0] == 0 {
                result.push(node_way);
            } else {
                new_nodes_way.append(&mut paths[node_way]);
            }
        }
        nodes_way = new_nodes_way;
    }
    result
}

fn solve(input: &mut Input, _test_case: usize) {
    // read graph
    let n: usize = input.read();
    let m: usize = input.read();

    let k: usize = input.read();
    let mut g = vec![vec![]; n];

    for _ in 0..m {
        let v1 = input.read::<usize>() - 1;
        let v2 = input.read::<usize>() - 1;
        g[v1].push(v2);
        g[v2].push(v1);
    }

    let c = input.read_vec::<u64>(n);

    // find shortest paths and present them as directed graph
    let sg = shortest_paths(&g, &c);

    // the answer is k-1 neighbors of node 1 and node 1

    if n == 2 && k == 1 && sg[0] == n - 1 {
        out_line!(-1);
    } else if k == 0 {
        out_line!();
    } else if k == 1 && sg[0] == n - 1 {
        out_line!(2);
    } else {
        let mut in_answer = vec![false; n];
        let mut answer = vec![0];
        in_answer[0] = true;
        let a = sg.len().min(k - 1);
        for i in 0..a {
            answer.push(sg[i]);
            in_answer[sg[i]] = true;
        }
        let mut nf = (k - 1) - a;
        let mut i = 0;
        while nf != 0 {
            if !in_answer[i] {
                answer.push(i);
                nf -= 1;
            }
            i += 1;
        }
        out_line!(answer.iter().map(|&x| x + 1).collect::<Vec<usize>>());
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let now = std::time::Instant::now();
    let t = input.read();
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
