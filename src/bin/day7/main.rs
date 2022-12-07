#[derive(Debug)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<File<'a>>,
    directories: Vec<Directory<'a>>,
}
impl<'a> Directory<'a> {
    fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum::<u64>()
            + self.directories.iter().map(|d| d.size()).sum::<u64>()
    }
}

#[derive(Debug)]
struct File<'a> {
    #[allow(dead_code)]
    name: &'a str,
    size: u64,
}

fn main() {
    let input = include_str!("input.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut root = Directory {
        name: "/",
        directories: Vec::new(),
        files: Vec::new(),
    };
    parse_directory(&mut root, &lines, 1);
    part_1(&root);
    part_2(&root);
}

fn part_2(root: &Directory) {
    let disk_space = 70000000;
    let needed_space = 30000000;
    let used_space = root.size();
    let left_space = disk_space - used_space;
    let freed_space = needed_space - left_space;
    let mut min_dir_space = u64::MAX;
    add_viable_dir(root, &mut min_dir_space, freed_space);
    println!("Part 2: {}", min_dir_space);
}

fn add_viable_dir(dir: &Directory, min_dir_space: &mut u64, threshold: u64) {
    if dir.size() < *min_dir_space && dir.size() > threshold {
        *min_dir_space = dir.size();
    }
    dir.directories
        .iter()
        .for_each(|d| add_viable_dir(d, min_dir_space, threshold));
}

fn part_1(root: &Directory) {
    let mut dir_under_100k = vec![];
    get_directory_under(root, 100000, &mut dir_under_100k);
    println!(
        "Part 1: {}",
        dir_under_100k.iter().fold(0, |acc, (_, s)| acc + s)
    );
}

fn get_directory_under<'a>(dir: &Directory<'a>, max_size: u64, store: &mut Vec<(&'a str, u64)>) {
    if dir.size() < max_size {
        store.push((dir.name, dir.size()));
    }
    dir.directories
        .iter()
        .for_each(|d| get_directory_under(d, max_size, store));
}

fn parse_directory<'a>(
    current_directory: &mut Directory<'a>,
    input: &[&'a str],
    idx: usize,
) -> usize {
    let mut files = vec![];
    let mut directories: Vec<Directory> = vec![];
    let mut idx = idx;
    while let Some(line) = input.get(idx) {
        idx += 1;
        if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("$ cd ..") {
            current_directory.directories.extend(directories);
            current_directory.files.extend(files);
            return idx;
        } else if line.starts_with("$ cd") {
            let dir = directories
                .iter_mut()
                .find(|d| d.name == line.split(' ').nth(2).unwrap())
                .unwrap();
            idx = parse_directory(dir, input, idx);
            continue;
        }
        let mut split = line.split(' ');
        let (p1, name) = (split.next().unwrap(), split.next().unwrap());
        if p1 == "dir" {
            directories.push(Directory {
                name,
                files: vec![],
                directories: vec![],
            });
        } else {
            files.push(File {
                name,
                size: p1.parse().unwrap(),
            })
        }
    }
    current_directory.directories.extend(directories);
    current_directory.files.extend(files);
    idx
}
