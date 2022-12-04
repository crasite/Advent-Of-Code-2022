#[derive(Debug)]
struct Bound {
    lower: usize,
    upper: usize,
}

fn main() {
    let elf_pairs = parse(include_str!("input.txt"));
    part_1(&elf_pairs);
    part_2(&elf_pairs);
}

fn part_1(elf_pairs: &[(Bound, Bound)]) {
    let count = elf_pairs.iter().fold(0, |acc, pair| {
        if is_pair_contained(&pair.0, &pair.1) {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 1: {}", count)
}
fn part_2(elf_pairs: &[(Bound, Bound)]) {
    let count = elf_pairs.iter().fold(0, |acc, pair| {
        if is_pair_overlapped(&pair.0, &pair.1) {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 2: {}", count)
}

fn parse(input: &str) -> Vec<(Bound, Bound)> {
    let mut rs = vec![];
    for line in input.lines() {
        let mut elf_jobs = line.split(',');
        let mut first_elf_bound = elf_jobs.next().unwrap().split('-');
        let first_elf_bound = Bound {
            lower: first_elf_bound.next().unwrap().parse().unwrap(),
            upper: first_elf_bound.next().unwrap().parse().unwrap(),
        };
        let mut second_elf_bound = elf_jobs.next().unwrap().split('-');
        let second_elf_bound = Bound {
            lower: second_elf_bound.next().unwrap().parse().unwrap(),
            upper: second_elf_bound.next().unwrap().parse().unwrap(),
        };
        rs.push((first_elf_bound, second_elf_bound));
    }
    rs
}

fn is_pair_contained(p1: &Bound, p2: &Bound) -> bool {
    (p1.lower >= p2.lower && p1.upper <= p2.upper) || (p2.lower >= p1.lower && p2.upper <= p1.upper)
}

fn is_pair_overlapped(p1: &Bound, p2: &Bound) -> bool {
    p1.lower <= p2.upper && p2.lower <= p1.upper
}
