//{"name":"susenky","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"susenky"}}}

use std::time::Instant;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let k = input.read::<usize>();

    let mut s = vec![vec![]; n];
    for i in 0..n {
        s[i] = input.read_vec::<u64>(k);
    }

    let m = 2<<(k-1);

    let mut dp = vec![vec![(0, vec![0; k]); m]; 2];

    dp[0][0].0 = *s[0].iter().max().unwrap();
    for i in 0..k {
        dp[0][1 << i].0 = s[0][i];
        dp[0][1 << i].1[i] = 0;
    }

    for i in 1..n {
        for j in 0..m {
            let mut result: (u64, Option<usize>) = (0, None);
            if dp[(i + 1)%2][j].0 != 0 {
                result = (dp[(i + 1)%2][j].0 + *s[i].iter().max().unwrap(), None);
            }
            for l in 0..k {
                let br = 1 << l;
                if j & br != 0 {
                    if dp[(i+1)%2][j-br].0 == 0 {
                        continue;
                    }
                    let candidate = s[i][l] + dp[(i + 1)%2][j - br].0;
                    if candidate > result.0 {
                        result = (candidate, Some(l));
                    }
                }
            }
            dp[i%2][j].0 = result.0;
            if let Some(si) = result.1 {
                dp[i%2][j].1 = dp[(i + 1)%2][j - (1 << si)].1.clone();
                dp[i%2][j].1[si] = i;
            } else {
                dp[i%2][j].1 = dp[(i + 1)%2][j].1.clone();
            }
        }
    }

    let answer = dp[(n - 1)%2][m-1].clone();
    out_line!(answer.0);
    for i in 0..n {
        if let Some(pos) = answer.1.iter().position(|&x| x == i) {
            out!(pos + 1, "");
        } else {
            let mx = s[i].iter().max().unwrap();

            out!(s[i].iter().position(|x| x == mx).unwrap() + 1, "");
        }
    }
    out_line!();
}

pub(crate) fn run(mut input: Input) -> bool {
    let now = Instant::now();
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    out_line!("millis:", now.elapsed().as_millis());
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
