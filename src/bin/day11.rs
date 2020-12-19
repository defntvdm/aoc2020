#[derive(Clone)]
enum State {
    Empty,
    Floor,
    Occ,
}

const STEPS: [(i64, i64); 8] = [
    (1, -1),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

fn main() {
    let mut field = get_field();
    let mut changed: bool = true;
    while changed {
        let (f, c) = step(&field, false, 3);
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

    field = get_field();
    changed = true;
    while changed {
        let (f, c) = step(&field, true, 4);
        field = f;
        changed = c;
    }
    occ = 0;
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

fn get_field() -> Vec<Vec<State>> {
    include_str!("../../input/d11.txt")
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
        .collect()
}

fn step(f: &Vec<Vec<State>>, long: bool, limit: i64) -> (Vec<Vec<State>>, bool) {
    let mut changed: bool = false;
    let mut nf: Vec<Vec<State>> = Vec::new();
    for (i, line) in f.iter().enumerate() {
        let mut nl: Vec<State> = Vec::new();
        for (j, s) in line.iter().enumerate() {
            if let State::Floor = s {
                nl.push(State::Floor);
                continue;
            }

            let mut occ: i64 = 0;
            for (x, y) in STEPS.iter() {
                let sx = *x;
                let sy = *y;
                let mut x = i as i64;
                let mut y = j as i64;
                loop {
                    x += sx;
                    y += sy;
                    if 0 <= x && x < f.len() as i64 && 0 <= y && y < line.len() as i64 {
                        match f[x as usize][y as usize] {
                            State::Occ => { occ += 1; break; },
                            State::Empty => break,
                            _ => {},
                        }
                        if !long {
                            break;
                        }
                    } else {
                        break;
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
                if occ > limit
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
