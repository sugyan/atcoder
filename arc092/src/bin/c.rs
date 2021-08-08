use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
        cd: [(usize, usize); n],
    }
    let mut m = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            if ab[i].0 < cd[j].0 && ab[i].1 < cd[j].1 {
                m[i][j] = true;
            }
        }
    }
    let mut answer = 0;
    while m.iter().any(|row| row.iter().any(|&b| b)) {
        if let Some(i) = (0..n)
            .filter(|&i| m[i].iter().any(|&b| b))
            .min_by_key(|&i| m[i].iter().filter(|&b| *b).count())
        {
            if let Some(j) = (0..n)
                .filter(|&j| m[i][j])
                .min_by_key(|&j| (0..n).filter(|&k| m[k][j]).count())
            {
                answer += 1;
                (0..n).for_each(|k| m[i][k] = false);
                (0..n).for_each(|k| m[k][j] = false);
            }
        }
    }
    println!("{}", answer);
}
