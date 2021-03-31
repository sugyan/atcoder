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
    while let Some(curr) = vd.pop_front() {
        for &next in &graph[curr] {
            if next > 0 && answer[next] == 0 {
                answer[next] = curr + 1;
                vd.push_back(next);
            }
        }
    }
    if (1..n).all(|i| answer[i] > 0) {
        println!("Yes");
        (1..n).for_each(|i| println!("{}", answer[i]));
    } else {
        println!("No");
    }
}
