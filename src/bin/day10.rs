fn main() {
    let mut inp: Vec<i64> = include_str!("../../input/d10.txt")
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
    inp.sort();

    let mut one = 1;
    let mut three = 1;
    for i in 0..inp.len() - 1 {
        match inp[i + 1] - inp[i] {
            1 => one += 1,
            3 => three += 1,
            _ => {}
        };
    }

    println!("{}", one * three);

    let mut dp: Vec<i128> = vec![0; inp.len()];
    for i in 0i64..inp.len() as i64 {
        if inp[i as usize] < 4 {
            dp[i as usize] = 1;
        }
        for j in i - 3..i {
            if j >= 0 {
                if inp[i as usize] - inp[j as usize] < 4 {
                    dp[i as usize] += dp[j as usize];
                }
            }
        }
    }

    println!("{}", dp[dp.len() - 1]);
}
