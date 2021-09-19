use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n: usize, m: usize,
    }
    let mut v = Vec::new();
    let mut map = vec![Vec::new(); n];
    for i in 0..m {
        input! {
            k: usize,
            a: [usize; k],
        }
        map[a[0] - 1].push(i);
        v.push(a);
    }
    let mut idx = vec![0; m];
    let mut stack = (0..n)
        .filter_map(|i| {
            if map[i].len() > 1 {
                Some([map[i][0], map[i][1]])
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    while let Some(last) = stack.pop() {
        for &i in &last {
            idx[i] += 1;
            if idx[i] < v[i].len() {
                map[v[i][idx[i]] - 1].push(i);
                if map[v[i][idx[i]] - 1].len() > 1 {
                    stack.push([map[v[i][idx[i]] - 1][0], map[v[i][idx[i]] - 1][1]]);
                }
            }
        }
        n -= 1;
    }
    println!("{}", if n == 0 { "Yes" } else { "No" });
}
