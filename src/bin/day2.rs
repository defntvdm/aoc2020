fn main() {
    let inp = include_str!("../../input/d2.txt");
    let mut count1: i64 = 0;
    let mut count2: i64 = 0;
    inp.lines().for_each(|l| {
        let lines: Vec<&str> = l.split(' ').collect();
        let limits: Vec<usize> = lines[0]
            .split('-')
            .map(|el| el.parse::<usize>().unwrap())
            .collect();
        let ch = lines[1].chars().nth(0).unwrap();
        let letter_count = lines[2].matches(ch).count();
        if limits[0] <= letter_count && letter_count <= limits[1] {
            count1 += 1;
        }

        let fch = lines[2].chars().nth(limits[0] - 1).unwrap();
        let lch = lines[2].chars().nth(limits[1] - 1).unwrap();
        if !(fch == ch && lch == ch || fch != ch && lch != ch) {
            count2 += 1
        }
    });
    println!("{}", count1);
    println!("{}", count2);
}
