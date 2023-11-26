fn main() {
    proconio::input! {
        s: proconio::marker::Chars,
    }

    for i in 0..s.len() - 1 {
        print!("{} ", s[i]);
    }
    print!("{}", s[s.len() - 1]);
}
