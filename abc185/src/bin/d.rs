use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32, m: usize,
        mut a: [i32; m],
    }
    a.push(0);
    a.push(n + 1);
    a.sort();
    let v = a.windows(2).map(|w| w[1] - w[0] - 1).collect::<Vec<_>>();
    let answer = if let Some(min) = v.iter().filter(|&e| *e != 0).min() {
        v.iter()
            .map(|&e| e / min + if e % min > 0 { 1 } else { 0 })
            .sum::<i32>()
    } else {
        0
    };
    println!("{}", answer);
}
