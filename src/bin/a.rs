#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut skip = false;
    let mut count = 0;
    for i in 0..s.len() - 1 {
        if skip {
            skip = false;
            continue;
        }
        if s[i] == s[i + 1] {
            count += 1;
            skip = true;
        }
    }
    count = count * k;
    if s[0] == s[s.len() - 1] && skip == false {
        count += (k - 1) / 2;
    }
    println!("{}", count);
}
