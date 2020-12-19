enum Direction {
    North,
    South,
    East,
    West,
}

enum Action {
    RotateRight,
    RotateLeft,
    MoveForward,
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
}

struct Step {
    action: Action,
    value: i64,
}

fn main() {
    let steps: Vec<Step> = include_str!("../../input/d12.txt")
        .lines()
        .map(|line| {
            let val: i64 = line[1..].parse::<i64>().unwrap();
            let mut step = Step {
                action: Action::MoveForward,
                value: val,
            };
            match line.chars().nth(0).unwrap() {
                'N' => step.action = Action::MoveNorth,
                'S' => step.action = Action::MoveSouth,
                'E' => step.action = Action::MoveEast,
                'W' => step.action = Action::MoveWest,
                'R' => step.action = Action::RotateRight,
                'L' => step.action = Action::RotateLeft,
                _ => {},
            }

            step
        })
        .collect();

    star1(&steps);
    star2(&steps);
}

fn star2(steps: &Vec<Step>) {
    let mut dx = 10;
    let mut dy = 1;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for step in steps {
        match step.action {
            Action::MoveWest => dx -= step.value,
            Action::MoveEast => dx += step.value,
            Action::MoveNorth => dy += step.value,
            Action::MoveSouth => dy -= step.value,
            Action::MoveForward => {
                x += dx * step.value;
                y += dy * step.value;
            },
            Action::RotateLeft => {
                let mut val = step.value;
                while val != 0 {
                    let tmp = -dy;
                    dy = dx;
                    dx = tmp;
                    val -= 90;
                }
            },
            Action::RotateRight => {
                let mut val = step.value;
                while val != 0 {
                    let tmp = -dx;
                    dx = dy;
                    dy = tmp;
                    val -= 90;
                }
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}

fn star1(steps: &Vec<Step>) {
    let mut direction = Direction::East;
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    for step in steps {
        match step.action {
            Action::MoveWest => x -= step.value,
            Action::MoveEast => x += step.value,
            Action::MoveNorth => y += step.value,
            Action::MoveSouth => y -= step.value,
            Action::MoveForward => {
                match direction {
                    Direction::West => x -= step.value,
                    Direction::East => x += step.value,
                    Direction::North => y += step.value,
                    Direction::South => y -= step.value,
                };
            },
            Action::RotateLeft => {
                let mut val = step.value;
                while val != 0 {
                    match direction {
                        Direction::West => direction = Direction::South,
                        Direction::East => direction = Direction::North,
                        Direction::North => direction = Direction::West,
                        Direction::South => direction = Direction::East,
                    }
                    val -= 90;
                }
            },
            Action::RotateRight => {
                let mut val = step.value;
                while val != 0 {
                    match direction {
                        Direction::West => direction = Direction::North,
                        Direction::East => direction = Direction::South,
                        Direction::North => direction = Direction::East,
                        Direction::South => direction = Direction::West,
                    }
                    val -= 90;
                }
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}
