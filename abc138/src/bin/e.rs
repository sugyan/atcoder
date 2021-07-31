use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut index = vec![Vec::new(); 26];
    for (i, &u) in s.as_bytes().iter().enumerate() {
        index[(u - b'a') as usize].push(i);
    }
    let mut answer = 0;
    let mut i = 0;
    for &u in t.as_bytes() {
        let v = &index[(u - b'a') as usize];
        if v.is_empty() {
            println!("-1");
            return;
        }
        i = match v.binary_search(&i) {
            Ok(j) => v[j] + 1,
            Err(j) => {
                if j < v.len() && i < v[j] {
                    v[j] + 1
                } else {
                    answer += s.len();
                    v[0] + 1
                }
            }
        };
    }
    answer += i;
    println!("{}", answer);
}
