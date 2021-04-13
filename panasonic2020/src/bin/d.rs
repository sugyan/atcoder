use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize }
    let mut answer = Vec::new();
    let mut v = vec![b'a'];
    recurse(&mut answer, &mut v, n - 1);
    for w in &answer {
        println!("{}", w);
    }
}

fn recurse(answer: &mut Vec<String>, v: &mut Vec<u8>, depth: usize) {
    if depth == 0 {
        answer.push(v.iter().map(|&b| b as char).collect());
        return;
    }
    if let Some(&max) = v.iter().max() {
        for b in b'a'..=max + 1 {
            v.push(b);
            recurse(answer, v, depth - 1);
            v.pop();
        }
    }
}
