use std::f64::consts::PI;

// 0時0分の時の角度を90度(PI/2)とし、そこから現在時刻で何度になっているか計算する。
// その後、その角度からcos,sinでxy座標を計算し、最後に2点間の距離を計算する。
fn main() {
    proconio::input! {
      a: f64,
      b: f64,
      h: f64,
      m: f64,
    }
    let h_theta = (PI / 2.0) - (h / 12.0 + m / (12.0 * 60.0)) * 2.0 * PI;
    let m_theta = (PI / 2.0) - (m / 60.0) * 2.0 * PI;

    let hx = a * h_theta.cos();
    let hy = a * h_theta.sin();
    let mx = b * m_theta.cos();
    let my = b * m_theta.sin();

    let distance = ((hx - mx).powf(2.0) + (hy - my).powf(2.0)).sqrt();
    println!("{}", distance);
}
