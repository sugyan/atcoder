use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut answer = std::i32::MIN;
    for i in 0..n {
        let (mut maxval, mut result) = (std::i32::MIN, 0);
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut points = (0, 0);
            for (k, &m) in a[i.min(j)..=i.max(j)].iter().enumerate() {
                if k % 2 == 0 {
                    points.0 += m;
                } else {
                    points.1 += m;
                }
            }
            if points.1 > maxval {
                result = points.0;
                maxval = points.1;
            }
        }
        answer = answer.max(result);
    }
    println!("{}", answer);
}
