use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize }
    let mut v = vec![b'a'];
    let mut answer = Vec::new();
    dfs(&mut v, n - 1, b'a', &mut answer);
    for s in &answer {
        println!("{}", s);
    }
}

fn dfs(v: &mut Vec<u8>, depth: usize, max: u8, answer: &mut Vec<String>) {
    if depth == 0 {
        answer.push(v.iter().map(|&b| b as char).collect());
        return;
    }
    for b in b'a'..=max + 1 {
        v.push(b);
        dfs(v, depth - 1, max.max(b), answer);
        v.pop();
    }
}
