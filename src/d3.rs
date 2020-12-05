static STEPS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

pub fn solve() {
    let lines: Vec<&str> = include_str!("../input/d3.txt").lines().collect();
    let mut result: [usize; 5] = [0; 5];
    for i in 0..STEPS.len() {
        let (sx, sy) = STEPS[i];
        let mut x: usize = 0;
        let mut y: usize = 0;
        while y < lines.len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                result[i] += 1;
            }
            x += sx;
            if x >= lines[y].len() {
                x -= lines[y].len();
            }
            y += sy;
        }
    }
    println!("{}", result[1]);
    println!("{}", result.iter().fold(1, |acc, el| acc * el));
}
