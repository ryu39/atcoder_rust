fn main() {
    proconio::input! {
      n: i32,
      x: i32,
      t: i32,
    }
    let mut required_num = n / x;
    if n % x != 0 {
        required_num += 1;
    }
    println!("{}", t * required_num);
}
