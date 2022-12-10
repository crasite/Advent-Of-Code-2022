#[derive(Debug)]
struct Command<'a> {
    op: &'a str,
    val: Option<isize>,
}

struct State<'a> {
    current_cycle: isize,
    x_register: isize,
    pending_cycle: usize,
    pending_command: &'a Command<'a>,
}

fn main() {
    let commands = parse(include_str!("input.txt"));
    run_commands(&commands, 20, 40);
    run_commands_state(
        &commands,
        0,
        1,
        0,
        &Command {
            op: "noop",
            val: None,
        },
    );
}

fn parse(input: &str) -> Vec<Command> {
    let mut commands = vec![];
    for line in input.lines() {
        let mut split = line.split(' ');
        let op = split.next().unwrap();
        let val = split.next().map(|v| v.parse().unwrap());
        commands.push(Command { op, val })
    }
    commands
}

fn run_commands_state<'a>(
    commands: &[Command<'a>],
    current_cycle: isize,
    x_register: isize,
    pending_cycle: usize,
    pending_command: &Command<'a>,
) -> State<'a> {
    println!("Current Cycle: {current_cycle}, X Register: {x_register}, Pending Cycle: {pending_cycle}, Pending Command: {pending_command:?}" );
    if pending_cycle > 0 {
        match pending_command.op {
            "noop" => run_commands_state(
                commands,
                current_cycle + 1,
                x_register,
                pending_cycle - 1,
                pending_command,
            ),
            "addx" => run_commands_state(
                commands,
                current_cycle + 1,
                x_register,
                pending_cycle - 1,
                pending_command,
            ),
            e => unreachable!("unknown command: {}", e),
        }
    } else {
        let mut new_x_register = x_register;
        match pending_command {
            Command { op: "noop", .. } => {}
            Command {
                op: "addx",
                val: Some(v),
            } => {
                new_x_register += v;
            }
            c => unreachable!("unknown command: {c:?}"),
        }
        if commands.first().is_none() {
            return;
        }
        let cmd = commands.first().unwrap();
        match cmd.op {
            "noop" => run_commands_state(
                commands.split_at(1).1,
                current_cycle + 1,
                new_x_register,
                0,
                cmd,
            ),
            "addx" => run_commands_state(
                commands.split_at(1).1,
                current_cycle + 1,
                new_x_register,
                1,
                cmd,
            ),
            e => unreachable!("unknown command: {}", e),
        }
    }
}

fn run_commands(
    commands: &[Command],
    cycle_stop_starting_point: isize,
    cycle_stop_interval: isize,
) {
    let mut register_x = 1;
    let mut cycle: isize = 0;
    let mut strength: isize = 0;
    for command in commands {
        if cycle > 220 {
            break;
        }
        match command.op {
            "noop" => {
                if cycle + 1 == cycle_stop_starting_point {
                    strength += (cycle + 1) * register_x;
                }
                if cycle > cycle_stop_starting_point
                    && (cycle + 1 - cycle_stop_starting_point) % cycle_stop_interval == 0
                {
                    strength += (cycle + 1) * register_x;
                }
                cycle += 1;
            }
            "addx" => {
                if cycle + 1 == cycle_stop_starting_point || cycle + 2 == cycle_stop_starting_point
                {
                    let cycle = if (cycle + 2) % cycle_stop_starting_point == 0 {
                        cycle + 2
                    } else {
                        cycle + 1
                    };
                    strength += cycle * register_x;
                }
                if cycle > cycle_stop_starting_point
                    && (cycle + 2 - cycle_stop_starting_point) % cycle_stop_interval <= 1
                {
                    let cycle =
                        if (cycle + 2 - cycle_stop_starting_point) % cycle_stop_interval == 0 {
                            cycle + 2
                        } else {
                            cycle + 1
                        };
                    strength += cycle * register_x;
                }
                register_x += command.val.unwrap();
                cycle += 2;
            }
            _ => unreachable!("Unknown Commands"),
        }
    }
    println!("Part 1: {}", strength);
}
