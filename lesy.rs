//{"name":"lesy","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"lesy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

use std::collections::BTreeMap;

fn get_area(trees: &mut BTreeMap<(usize, usize), bool>, &pos: &(usize, usize)) -> usize {
    let mut stack = vec![pos];
    let mut brx = [pos.0, pos.0 + 1];
    let mut bry = [pos.1, pos.1 + 1];

    while let Some(pos) = stack.pop() {
        brx[0] = brx[0].min(pos.0);
        brx[1] = brx[1].max(pos.0 + 1);
        bry[0] = bry[0].min(pos.1);
        bry[1] = bry[1].max(pos.1 + 1);
        let positions = [
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
            (pos.0 - 1, pos.1),
        ];
        for pos in positions {
            if let Some(fenced) = trees.get_mut(&pos) {
                if !*fenced {
                    stack.push(pos);
                    *fenced = true;
                }
            }
        }
    }
    return (brx[1] - brx[0]) * (bry[1] - bry[0]);
}

fn solve(input: &mut Input, _test_case: usize) {
    let m: usize = input.read();
    let n: usize = input.read();

    let mut trees = BTreeMap::new();

    let l: usize = input.read();
    for _ in 0..l {
        let pos = (input.read::<usize>(), input.read::<usize>());
        trees.insert(pos, false);
    }

    let mut areas = vec![];

    for pos in trees.clone().keys().into_iter() {
        if !trees.get(&pos).unwrap() {
            areas.push(get_area(&mut trees, pos));
        }
    }

    out_line!(areas.iter().sum::<usize>());
}

pub(crate) fn run(mut input: Input) -> bool {
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
