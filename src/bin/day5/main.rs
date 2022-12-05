use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Stack {
    crates: VecDeque<char>,
}

#[derive(Debug)]
struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (mut stack_list, command_list) = parse(include_str!("input.txt"));
    draw(&stack_list);
    part_1(&mut stack_list.clone(), &command_list);
    part_2(&mut stack_list, &command_list);
}

fn parse(input: &str) -> (Vec<Stack>, Vec<Command>) {
    let mut total_stack = 0;
    //finding total stack (Actually unneeded but I'm too tired to fix)
    for line in input.lines() {
        let mut cur_idx = 1;
        if line.get(cur_idx..cur_idx + 1) == Some("1") {
            cur_idx += 4;
            while line.get(cur_idx..cur_idx + 1).is_some() {
                cur_idx += 4;
            }
            total_stack = (cur_idx - 1) / 4;
            break;
        }
    }
    let mut stack_list: Vec<Stack> = Vec::new();
    let mut command_list: Vec<Command> = Vec::new();
    for _ in 0..total_stack {
        stack_list.push(Stack {
            crates: VecDeque::new(),
        });
    }
    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let mut cur_idx = 1;
        while let Some(c) = line.get(cur_idx..cur_idx + 1) {
            cur_idx += 4;
            let c = c.chars().next().unwrap();
            if c.is_numeric() {
                break;
            }
            if c != ' ' {
                let stack_idx = ((cur_idx - 1) / 4) - 1;
                stack_list[stack_idx].crates.push_front(c);
            }
        }
    }
    for line in &mut lines {
        let mut command_part = line.split(' ');
        command_part.next();
        let amount = command_part.next().unwrap().parse::<usize>().unwrap();
        command_part.next();
        let from = command_part.next().unwrap().parse::<usize>().unwrap() - 1;
        command_part.next();
        let to = command_part.next().unwrap().parse::<usize>().unwrap() - 1;
        let command = Command { amount, from, to };
        command_list.push(command);
    }
    (stack_list, command_list)
}

fn part_1(stack_list: &mut Vec<Stack>, command_list: &Vec<Command>) {
    for command in command_list {
        let mut crates = Vec::new();
        for _ in 0..command.amount {
            crates.push(stack_list[command.from].crates.pop_back().unwrap());
        }
        for c in crates {
            stack_list[command.to].crates.push_back(c);
        }
        // draw(&stack_list);
    }
    for stack in stack_list {
        print!("{}", stack.crates.back().unwrap_or(&' '));
    }
    println!();
}

fn part_2(stack_list: &mut Vec<Stack>, command_list: &Vec<Command>) {
    for command in command_list {
        let mut crates = VecDeque::new();
        for _ in 0..command.amount {
            crates.push_front(stack_list[command.from].crates.pop_back().unwrap());
        }
        for c in crates {
            stack_list[command.to].crates.push_back(c);
        }
        // draw(&stack_list);
    }
    for stack in stack_list {
        print!("{}", stack.crates.back().unwrap_or(&' '));
    }
    println!();
}

fn draw(stack_list: &Vec<Stack>) {
    let mut max_len = 0;
    for stack in stack_list {
        if stack.crates.len() > max_len {
            max_len = stack.crates.len();
        }
    }
    for i in 0..max_len {
        for stack in stack_list {
            if stack.crates.len() > i {
                print!("{} ", stack.crates[i]);
            } else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
}
