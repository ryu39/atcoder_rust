fn main() {
    proconio::input! {
      a: i32
    }
    println!("{}", a + a.pow(2) + a.pow(3));
}
