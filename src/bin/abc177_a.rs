fn main() {
    proconio::input! {
        d: i32,
        t: i32,
        s: i32,
    }
    let d2 = t * s;
    if d <= d2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
