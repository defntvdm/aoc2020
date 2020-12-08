struct Command<'a> {
    cmd: &'a str,
    n: i64,
}

pub fn solve() {
    let commands: Vec<Command> = include_str!("../input/d8.txt")
        .lines()
        .map(|el| {
            let parts: Vec<&str> = el.trim().split(' ').collect();
            Command {
                cmd: &parts[0],
                n: parts[1].parse::<i64>().unwrap(),
            }
        })
        .collect();
    let mut visited_commands: Vec<bool> = vec![false; commands.len()];
    let mut current_op: i64 = 0;
    let mut acc: i64 = 0;
    while 0 <= current_op && current_op < visited_commands.len() as i64 && !visited_commands[current_op as usize] {
        visited_commands[current_op as usize] = true;
        match commands[current_op as usize].cmd {
            "acc" => {
                acc += commands[current_op as usize].n;
                current_op += 1;
            },
            "nop" => {
                current_op += 1;
            },
            "jmp" => {
                current_op += commands[current_op as usize].n;
            },
            _ => {},
        };
    }
    println!("{}", acc);
}
