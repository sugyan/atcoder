use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize, m: usize,
        a: [[usize]; m],
    }
    let mut map = vec![Vec::new(); n];
    for i in 0..m {
        map[a[i][0] - 1].push(i);
    }
    let mut idx = vec![0; m];
    let mut stack = (0..n).filter(|&i| map[i].len() > 1).collect::<Vec<_>>();
    while let Some(last) = stack.pop() {
        let v = map[last].clone();
        for &i in &v {
            idx[i] += 1;
            if idx[i] < a[i].len() {
                let j = a[i][idx[i]] - 1;
                map[j].push(i);
                if map[j].len() > 1 {
                    stack.push(j);
                }
            }
        }
        n -= 1;
    }
    println!("{}", if n == 0 { "Yes" } else { "No" });
}
