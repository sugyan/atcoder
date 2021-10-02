use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let answer = dfs(0, n);
    println!("{}", answer);
}

fn dfs(m: u64, n: u64) -> u32 {
    if m > n {
        return 0;
    }
    let mut ret = 0;
    if ["3", "5", "7"].iter().all(|c| m.to_string().contains(c)) {
        ret += 1;
    };
    [3, 5, 7].iter().for_each(|&d| ret += dfs(m * 10 + d, n));
    ret
}
