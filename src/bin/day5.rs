fn main() {
    let lines = include_str!("../../input/d5.txt").lines();
    let mut seat_ids: Vec<usize> = lines
        .map(|line| {
            let mut l = 0;
            let mut r = 127;
            for c in line[..7].chars() {
                let m = (l + r) / 2;
                match c {
                    'F' => r = m,
                    'B' => l = m + 1,
                    _ => {}
                }
            }
            l <<= 3;
            match &line[line.len() - 3..] {
                "LLL" => l += 0,
                "LLR" => l += 1,
                "LRL" => l += 2,
                "LRR" => l += 3,
                "RLL" => l += 4,
                "RLR" => l += 5,
                "RRL" => l += 6,
                "RRR" => l += 7,
                _ => {}
            }
            l
        })
        .collect();

    seat_ids.sort();
    println!("{}", seat_ids[seat_ids.len() - 1]);

    let min: usize = seat_ids[0];
    for i in 0..seat_ids.len() {
        if i + min != seat_ids[i] {
            println!("{}", i + min);
            break;
        }
    }
}
