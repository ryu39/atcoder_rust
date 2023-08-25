fn main() {
    proconio::input! {
        k: i32,
        a: i32,
        b: i32,
    }
    let max_k = b / k * k;
    if max_k >= a {
        println!("OK");
    } else {
        println!("NG");
    }
}
