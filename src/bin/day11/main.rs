#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: String,
    test_number: usize,
    true_branch: usize,
    false_branch: usize,
    inspected_count: usize,
}

fn main() {
    let input = include_str!("input.txt");
    part_1(input);
    part_2(input);
}

fn part_2(input: &str) {
    let mut monkeys = vec![];
    parse(input, &mut monkeys);
    for _ in 0..10000 {
        play_round(&mut monkeys, false);
    }
    let mut inspected_total = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .collect::<Vec<usize>>();
    inspected_total.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", inspected_total[0] * inspected_total[1]);
}
fn part_1(input: &str) {
    let mut monkeys = vec![];
    parse(input, &mut monkeys);
    for _ in 0..20 {
        play_round(&mut monkeys, true);
    }
    let mut inspected_total = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .collect::<Vec<usize>>();
    inspected_total.sort_by(|a, b| b.cmp(a));
    println!("Part 1: {}", inspected_total[0] * inspected_total[1]);
}

fn play_round(monkeys: &mut Vec<Monkey>, reduce_worry: bool) {
    let mod_number: usize = monkeys.iter().map(|m| m.test_number).product();
    for i in 0..monkeys.len() {
        let mut throw_list: Vec<(usize, usize)> = vec![];
        for item in monkeys[i].items.iter() {
            let mut operation = monkeys[i].operation.split(' ');
            let first_operand = match operation.next().unwrap() {
                "old" => *item,
                o => o.parse().unwrap(),
            };
            let op = operation.next().unwrap();
            let second_operand = match operation.next().unwrap() {
                "old" => *item,
                o => o.parse().unwrap(),
            };
            let mut new_value = match op {
                "+" => first_operand + second_operand,
                "*" => first_operand * second_operand,
                _ => panic!("Unknown operation: {}", op),
            };
            if reduce_worry {
                new_value /= 3;
            } else {
                new_value %= mod_number;
            }
            if new_value % monkeys[i].test_number == 0 {
                throw_list.push((monkeys[i].true_branch, new_value));
            } else {
                throw_list.push((monkeys[i].false_branch, new_value));
            }
        }
        monkeys[i].inspected_count += monkeys[i].items.len();
        monkeys[i].items.clear();
        for (idx, value) in throw_list {
            monkeys[idx].items.push(value);
        }
    }
}

fn parse(input: &str, monkeys: &mut Vec<Monkey>) {
    let mut lines = input.lines().map(|s| s.trim());
    while lines.next().map(|l| !l.is_empty()).unwrap_or(false) {
        let mut items: Vec<usize> = vec![];
        parse_starting_items(&mut lines, &mut items);
        let operation_raw = lines.next().unwrap();
        let operation = operation_raw.split('=').last().unwrap().trim().to_string();
        let test_raw = lines.next().unwrap();
        let test = test_raw.split(' ').last().unwrap().trim().parse().unwrap();
        let true_raw = lines.next().unwrap();
        let if_true = true_raw.split(' ').last().unwrap().trim().parse().unwrap();
        let false_raw = lines.next().unwrap();
        let if_false = false_raw.split(' ').last().unwrap().trim().parse().unwrap();
        lines.next();
        let monkey = Monkey {
            items,
            operation,
            test_number: test,
            true_branch: if_true,
            false_branch: if_false,
            inspected_count: 0,
        };
        monkeys.push(monkey);
    }
}

fn parse_starting_items<'a>(lines: &mut impl Iterator<Item = &'a str>, items: &mut Vec<usize>) {
    let starting_raw = lines.next().unwrap();
    let mut starting_raw = starting_raw.split([' ', ',']);
    starting_raw.next();
    starting_raw.next();
    for t in starting_raw {
        if !t.is_empty() {
            items.push(t.parse().unwrap());
        }
    }
}
