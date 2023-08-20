fn main() {
    proconio::input! {
      a: i32,
      b: i32
    }
    let following_limit = 2 * a + 100;
    println!("{}", following_limit - b);
}
