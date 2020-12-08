struct Command<'a> {
    cmd: &'a str,
    n: i64,
}

fn execute(commands: &Vec<Command>) -> (i64, bool) {
    let mut visited_commands: Vec<bool> = vec![false; commands.len()];
    let mut current_op: i64 = 0;
    let mut acc: i64 = 0;
    while 0 <= current_op
        && current_op < visited_commands.len() as i64
        && !visited_commands[current_op as usize]
    {
        visited_commands[current_op as usize] = true;
        match commands[current_op as usize].cmd {
            "acc" => {
                acc += commands[current_op as usize].n;
                current_op += 1;
            }
            "nop" => {
                current_op += 1;
            }
            "jmp" => {
                current_op += commands[current_op as usize].n;
            }
            _ => {}
        };
    }
    (acc, current_op == commands.len() as i64)
}

fn main() {
    let mut commands: Vec<Command> = include_str!("../../input/d8.txt")
        .lines()
        .map(|el| {
            let parts: Vec<&str> = el.trim().split(' ').collect();
            Command {
                cmd: &parts[0],
                n: parts[1].parse::<i64>().unwrap(),
            }
        })
        .collect();

    let (acc, _) = execute(&commands);

    // Star 1
    println!("{}", acc);

    for i in 0..commands.len() {
        match commands[i].cmd {
            "jmp" => commands[i].cmd = "nop",
            "nop" => commands[i].cmd = "jmp",
            _ => continue,
        };

        let (acc, full) = execute(&commands);
        if full {
            // Start 2
            println!("{}", acc);
            return;
        }

        match commands[i].cmd {
            "jmp" => commands[i].cmd = "nop",
            "nop" => commands[i].cmd = "jmp",
            _ => {}
        };
    }
}
