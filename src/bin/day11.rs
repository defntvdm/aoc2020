#[derive(Clone)]
enum State {
    Empty,
    Floor,
    Occ,
}

fn main() {
    let mut field: Vec<Vec<State>> = include_str!("../../input/d11.txt")
        .lines()
        .map(|line| {
            let mut v: Vec<State> = vec![State::Floor; line.len()];
            for (i, c) in line.chars().enumerate() {
                match c {
                    'L' => v[i] = State::Empty,
                    '#' => v[i] = State::Occ,
                    _ => {}
                }
            }

            return v;
        })
        .collect();

    let mut changed: bool = true;

    while changed {
        let (f, c) = star1(&field);
        field = f;
        changed = c;
    }

    let mut occ = 0;
    for line in &field {
        for s in line.iter() {
            match s {
                State::Occ => occ += 1,
                _ => {}
            }
        }
    }

    println!("{}", occ);
}

fn star1(f: &Vec<Vec<State>>) -> (Vec<Vec<State>>, bool) {
    let mut changed: bool = false;
    let mut nf: Vec<Vec<State>> = Vec::new();
    for (i, line) in f.iter().enumerate() {
        let mut nl: Vec<State> = Vec::new();
        for (j, s) in line.iter().enumerate() {
            if let State::Floor = s {
                nl.push(State::Floor);
                continue;
            }

            let neigs: [(i64, i64); 8] = [
                (i as i64 + 1, j as i64 - 1),
                (i as i64 + 1, j as i64),
                (i as i64 + 1, j as i64 + 1),
                (i as i64, j as i64 - 1),
                (i as i64, j as i64 + 1),
                (i as i64 - 1, j as i64 - 1),
                (i as i64 - 1, j as i64),
                (i as i64 - 1, j as i64 + 1),
            ];
            let mut occ: i64 = 0;
            for (x, y) in neigs.iter() {
                let x = *x;
                let y = *y;
                if 0 <= x && x < f.len() as i64 && 0 <= y && y < line.len() as i64 {
                    match f[x as usize][y as usize] {
                        State::Occ => occ += 1,
                        _ => {}
                    }
                }
            }

            if occ == 0
                && match s {
                    State::Empty => true,
                    _ => false,
                }
            {
                nl.push(State::Occ);
                changed = true;
            } else {
                if occ > 3
                    && match s {
                        State::Occ => true,
                        _ => false,
                    }
                {
                    nl.push(State::Empty);
                    changed = true;
                } else {
                    nl.push(s.clone());
                }
            }
        }
        nf.push(nl);
    }

    (nf, changed)
}
