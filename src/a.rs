use super::*;

pub fn solve() {
    let (n, ): (usize, ) = scanln();

    let mut v: Vec<u32> = scanln();
    v.sort();


    let mut answer = 0;
    for i in 0..n {
        if v[i] > i as u32 {
            answer = i;
            break;
        }
        answer = i+1;
    }

    outln!("{}", answer);

}
