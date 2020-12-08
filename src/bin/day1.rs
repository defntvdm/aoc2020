const YEAR: i64 = 2020;

fn main() {
    let mut numbers: Vec<i64> = include_str!("../../input/d1.txt")
        .lines()
        .map(|a| a.parse::<i64>().unwrap())
        .collect();
    numbers.sort();
    for a in numbers.iter() {
        let b = YEAR - a;
        if let Ok(_) = numbers.binary_search(&b) {
            println!("{} + {} = {}; {} * {} = {}", a, b, YEAR, a, b, a * b);
            break;
        }
    }

    'cycle: for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = numbers[i];
            let b = numbers[j];
            let c = YEAR - a - b;
            if let Ok(_) = numbers.binary_search(&c) {
                println!(
                    "{} + {} + {} = {}; {} * {} * {} = {}",
                    a,
                    b,
                    c,
                    YEAR,
                    a,
                    b,
                    c,
                    a * b * c
                );
                break 'cycle;
            }
        }
    }
}
