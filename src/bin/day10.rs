fn main() {
    let mut inp: Vec<i64> = include_str!("../../input/d10.txt")
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
    inp.sort();
    let mut one = 1;
    let mut three = 1;
    for i in 0..inp.len() - 1 {
        match inp[i+1] - inp[i] {
            1 => one += 1,
            3 => three += 1,
            _ => {}
        };
    }

    println!("{}", one * three);
}
