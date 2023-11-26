use std::collections::VecDeque;

fn check(target: &Vec<char>, stamp: &Vec<char>, start: i32) -> bool {
    for i in 0..stamp.len() {
        let j = (start + i as i32) as usize;
        if target[j] != stamp[i] && target[j] != '#' {
            return false;
        }
    }
    true
}

fn main() {
    proconio::input! {
      n: i32,
      m: i32,
      mut s: proconio::marker::Chars,
      t: proconio::marker::Chars,
    }

    let mut queue = VecDeque::new();
    let mut checked = vec![false; (n - m + 1) as usize];

    for i in 0..=(n - m) {
        if check(&s, &t, i) {
            checked[i as usize] = true;
            queue.push_back(i);
        }
    }

    while !queue.is_empty() {
        let i = queue.pop_front().unwrap();

        for j in 0..m {
            s[(i + j) as usize] = '#';
        }

        for j in std::cmp::max(i - (m - 1), 0)..=std::cmp::min(i + (m - 1), n - m) {
            if checked[j as usize] {
                continue;
            }

            if check(&s, &t, j) {
                checked[j as usize] = true;
                queue.push_back(j);
            }
        }
    }

    for i in 0..n {
        if s[i as usize] != '#' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
