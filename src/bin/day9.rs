use std::collections::HashSet;

fn main() {
    let inp: Vec<i64> = include_str!("../../input/d9.txt")
        .lines()
        .map(|el| el.parse::<i64>().unwrap())
        .collect();

    let mut res = 0i64;
    let mut window: HashSet<i64> = HashSet::new();

    for k in 0..25 {
        window.insert(inp[k]);
    }

    for k in 25..inp.len() {
        let mut ok = false;
        for e in &window {
            if *e == inp[k] - e {
                continue;
            }
            if window.contains(&(inp[k] - e)) {
                ok = true;
                break;
            }
        }
        if !ok {
            res = inp[k];
            break;
        }
        window.remove(&inp[k-25]);
        window.insert(inp[k]);
    }

    println!("{}", res);

    let mut s = inp[0];
    let mut l = 0;
    let mut r = 0;
    while s != res {
        if s < res {
            r += 1;
            s += inp[r];
        } else {
            s -= inp[l];
            l += 1;
        }
    }
    let res_slice = &inp[l..=r];
    println!("{}", res_slice.iter().min().unwrap() + res_slice.iter().max().unwrap());
}
