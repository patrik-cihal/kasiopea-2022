use super::*;

pub fn solve() {
    let (k, ): (usize, ) = scanln();
    let (n, base_queue): (usize, String) = scanln();
    let base_queue: Vec<char> = base_queue.chars().into_iter().collect();

    let mut queues = vec![vec![]; k];
    for i in 0..k {
        let (n, raw_queue): (usize, String) = scanln();
        queues[i] = raw_queue.chars().into_iter().collect::<Vec<char>>();
    }

    // read and process first n values by saving to stack
    let mut stack = vec![];
    for val in base_queue {
        if let Some(prev_val) = stack.pop() {
            if prev_val != val {
                stack.push(prev_val); 
                stack.push(val);
            }
        }
        else {
            stack.push(val);
        }
    }
    // this stack is now the one we will be returning to after each queue processed
    // process each queue and save operations performed such that we can reverse them afterwards
    let mut answer = stack.len()+queues[0].len();
    for i in 0..k {
        let mut operations = vec![];
        for (i, val) in queues[i].iter().enumerate() {
            if let Some(prev_val) = stack.pop() {
                if &prev_val != val {
                    stack.push(prev_val); 
                    stack.push(*val);
                }
                operations.push(i);
            }
            else {
                stack.push(*val);
            }
        }
        // update answer based on final queue size
        answer = answer.min(stack.len());
        // reverse operations
        for (i, val) in queues[i].iter().enumerate() {
            if let Some(last_operation) = operations.pop() {
                if last_operation != i {
                    operations.push(last_operation);
                }
                else {
                    stack.push(*val);
                    stack.push(*val);
                }
            }
            stack.pop();
        }
    }
    outln!("{}", answer);
}
