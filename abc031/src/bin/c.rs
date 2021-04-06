use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut answer = std::i32::MIN;
    for i in 0..n {
        let results = (0..n)
            .filter_map(|j| {
                let mut points = (0, 0);
                for (k, &m) in a[i.min(j)..=i.max(j)].iter().enumerate() {
                    if k % 2 == 0 {
                        points.0 += m;
                    } else {
                        points.1 += m;
                    }
                }
                Some(points).filter(|_| i != j)
            })
            .collect::<Vec<_>>();
        if let Some(&max) = results.iter().max_by_key(|&p| p.1) {
            answer = answer.max(results.iter().find(|&p| p.1 == max.1).unwrap().0);
        }
    }
    println!("{}", answer);
}
