//{"name":"bakterie","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"bakterie"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

struct Range {
    start: i64,
    end: i64,
    reach: i64,
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let k = input.read::<i64>();

    let mut coords = vec![];
    for _ in 0..n {
        coords.push((input.read::<i64>(), input.read::<i64>()));
    }
    coords.sort();

    let mut answer = n as i64 * (k * 2 + 1).pow(2);

    for i in 0..n {
        let mut left = coords[0..i].to_owned();
        left.sort_by(|a, b| a.1.cmp(&b.1));
        let mut ranges: Vec<Range> = vec![];
        for coord in left {
            let mut new_range = Range {
                start: coord.1 - k,
                end: coord.1 + k,
                reach: coord.0 + k,
            };
            while let Some(mut lr) = ranges.pop() {
                if lr.end >= new_range.start {
                    if lr.reach > new_range.reach {
                        new_range.start = lr.end + 1;
                    } else {
                        lr.end = new_range.start - 1;
                        if lr.end < lr.start {
                            continue;
                        }
                    }
                }
                ranges.push(lr);
                break;
            }
            ranges.push(new_range);
        }

        for range in ranges {
            let start = range.start.max(coords[i].1 - k);
            let end = range.end.min(coords[i].1 + k);
            let sub = range.reach + 1 - (coords[i].0 - k);
            if sub > 0 && start <= end {
                answer -= (end + 1 - start) * sub;
            }
        }
    }

    out_line!(answer);
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
