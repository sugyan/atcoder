use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize, usize); m],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }
    let mut answer = vec![0; n];
    let mut vd = VecDeque::new();
    vd.push_back(0);
    while let Some(front) = vd.pop_front() {
        for &n in &graph[front] {
            if n > 0 && answer[n] == 0 {
                answer[n] = front + 1;
                vd.push_back(n);
            }
        }
    }
    if answer[1..].iter().all(|&n| n > 0) {
        println!("Yes");
        answer[1..].iter().for_each(|&n| println!("{}", n));
    } else {
        println!("No");
    }
}
