use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug, Clone)]
struct Scanner {
    points: Vec<Point>,
}

fn main() {
    let input = include_str!("input.txt");
    let lines_iter = input.lines();
    let mut scanner_list: Vec<Scanner> = vec![];
    let mut scanner_location: Vec<Point> = vec![Point { x: 0, y: 0, z: 0 }];
    create_scanner_list(lines_iter, &mut scanner_list);
    let mut aligned_point = HashSet::new();
    aligned_point.extend(scanner_list.pop().unwrap().points);
    while !scanner_list.is_empty() {
        scanner_list = scanner_list
            .iter()
            .filter(|scanner| {
                !rotate_six_direction(&scanner.points, &mut aligned_point, &mut scanner_location)
            })
            .cloned()
            .collect::<Vec<Scanner>>();
        dbg!(scanner_list.len());
        dbg!(&scanner_location);
    }
    dbg!(aligned_point.len());
    dbg!(largest_manhattan_distance(&scanner_location));
}

fn largest_manhattan_distance(points: &Vec<Point>) -> i32 {
    let mut largest = 0;
    for point1 in points {
        for point2 in points {
            let distance = (point1.x - point2.x).abs()
                + (point1.y - point2.y).abs()
                + (point1.z - point2.z).abs();
            if distance > largest {
                largest = distance;
            }
        }
    }
    largest
}

fn rotate_six_direction(
    points: &[Point],
    aligned_point_set: &mut HashSet<Point>,
    scanner_location: &mut Vec<Point>,
) -> bool {
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: p.y,
                z: p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: -p.y,
                y: p.x,
                z: p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: -p.x,
                y: -p.y,
                z: p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: p.y,
                y: -p.x,
                z: p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: p.z,
                y: p.y,
                z: -p.x,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if rotate_four_rotation(
        &points
            .iter()
            .map(|p| Point {
                x: -p.z,
                y: p.y,
                z: p.x,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    false
}

fn rotate_four_rotation(
    points: &[Point],
    aligned_point_set: &mut HashSet<Point>,
    scanner_location: &mut Vec<Point>,
) -> bool {
    if match_with_correct_point(
        &points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: p.y,
                z: p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if match_with_correct_point(
        &points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: p.z,
                z: -p.y,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if match_with_correct_point(
        &points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: -p.y,
                z: -p.z,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    if match_with_correct_point(
        &points
            .iter()
            .map(|p| Point {
                x: p.x,
                y: -p.z,
                z: p.y,
            })
            .collect::<Vec<Point>>(),
        aligned_point_set,
        scanner_location,
    ) {
        return true;
    }
    false
}

fn match_with_correct_point(
    point_list: &[Point],
    aligned_point_set: &mut HashSet<Point>,
    scanner_location: &mut Vec<Point>,
) -> bool {
    let point_set_clone = aligned_point_set.clone();
    for point in point_list.iter() {
        for aligned_point in point_set_clone.iter() {
            let x_offset = aligned_point.x - point.x;
            let y_offset = aligned_point.y - point.y;
            let z_offset = aligned_point.z - point.z;
            let mut total_match = 0;
            let aligned_point = point_list
                .iter()
                .map(|p| Point {
                    x: p.x + x_offset,
                    y: p.y + y_offset,
                    z: p.z + z_offset,
                })
                .collect::<Vec<Point>>();
            aligned_point.iter().for_each(|p| {
                if point_set_clone.contains(p) {
                    total_match += 1;
                }
            });
            if total_match >= 12 {
                aligned_point_set.extend(aligned_point);
                scanner_location.push(Point {
                    x: x_offset,
                    y: y_offset,
                    z: z_offset,
                });
                return true;
            }
        }
    }
    false
}

fn create_scanner_list(lines_iter: std::str::Lines, scanner_list: &mut Vec<Scanner>) {
    let mut scanner = vec![];
    for line in lines_iter {
        if line.starts_with("---") {
            if !scanner.is_empty() {
                scanner_list.push(Scanner {
                    points: scanner.clone(),
                });
                scanner.clear();
            }
            continue;
        }
        if !line.is_empty() {
            let mut coordinate = line.split(',');
            let x = coordinate.next().unwrap().parse::<i32>().unwrap();
            let y = coordinate.next().unwrap().parse::<i32>().unwrap();
            let z = coordinate.next().unwrap().parse::<i32>().unwrap();
            scanner.push(Point { x, y, z });
        }
    }
    scanner_list.push(Scanner {
        points: scanner.clone(),
    });
}
