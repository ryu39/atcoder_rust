fn main() {
    proconio::input! {
        k: i32,
        a: i32,
        b: i32,
    }
    if a % k == 0 || b % k == 0 {
        println!("OK");
        return;
    }

    let diff = b - a;
    if ((b % k) - (a % k)) != diff {
        println!("OK");
    } else {
        println!("NG");
    }
}
