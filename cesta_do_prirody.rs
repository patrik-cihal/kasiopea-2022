//{"name":"cesta_do_prirody","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"cesta_do_prirody"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashMap;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut flower_collection = HashMap::new();

    for _ in 0..n {
        let flower = input.read::<String>();
        let count: u64 = input.read();
        if let Some(flower_count) = flower_collection.get_mut(&flower) {
            *flower_count += count;
        } else {
            flower_collection.insert(flower, count);
        }
    }

    out_line!(flower_collection.len());
    for (key, value) in flower_collection.into_iter() {
        out_line!(key, value);
    }
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
