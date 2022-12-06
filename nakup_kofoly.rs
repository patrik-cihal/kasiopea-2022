//{"name":"nakup_kofoly","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"nakup_kofoly"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let c1: u32 = input.read();
    let v1: u32 = input.read();
    let c2: u32 = input.read();
    let v2: u32 = input.read();

    if c2*v1 < c1*v2  {
        out_line!("MENSI");
    }
    else {
        out_line!("VETSI");
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
