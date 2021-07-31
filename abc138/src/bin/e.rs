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
    let mut a = 0;
    let mut i = 0;
    for &u in t.as_bytes() {
        let v = &index[(u - b'a') as usize];
        if v.is_empty() {
            println!("-1");
            return;
        }
        let j = match v.binary_search(&i) {
            Ok(j) => v[j],
            Err(j) if j < v.len() => v[j],
            _ => v[0],
        };
        if j < i {
            a += s.len();
        }
        i = j + 1;
    }
    println!("{}", a + i);
}
