use super::*;

fn go(n: usize) -> Vec<char> {
    if n == 1 {
        return vec!['1'];
    }
    let mut result = vec![];
    result.extend(vec!['(', '1', '+', '1', ')']);
    if n > 2 {
        result.push('*');
        result.push('(');
        result.extend(go(n/2));
        result.push(')');
    }
    if n%2 == 1 {
        result.push('+');
        result.push('1');
    }
    result
}


pub fn solve() {
    let (n, ): (usize, ) = scanln();
    let answer = go(n);
    println!("{:?}", answer);
}
