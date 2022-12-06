//{"name":"piktogramy","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"piktogramy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn count_ones(mut val: u32) -> u8 {
    let mut answer = 0;
    while val != 0 {
        answer += val % 2;
        val /= 2;
    }
    answer as u8
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();
    let mut p = vec![vec![]; n];
    for i in 0..n {
        let m = input.read::<usize>().clone();
        p[i] = input.read_vec(m).iter().map(|&x| count_ones(x)).collect();
    }

    let mut sums = vec![0u64; k + 1];
    sums[0] = 1;
    for i in 0..n {
        let mut new_sums = vec![0; k + 1];
        for val in &p[i] {
            for j in 0..(k + 1 - (*val as usize).min(k + 1)) {
                new_sums[j + *val as usize] += sums[j];
            }
        }
        sums = new_sums;
    }
    out_line!(sums[k]);
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
