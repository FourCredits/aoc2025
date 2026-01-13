use std::{
    cmp::Reverse,
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Point = (u32, u32, u32);
type Pairs = Vec<((u32, u32, u32), (u32, u32, u32))>;
type Circuits = Vec<HashSet<(u32, u32, u32)>>;

pub fn parse(path: impl AsRef<Path>) -> (Pairs, Circuits) {
    let reader = BufReader::new(File::open(path).unwrap());
    let junction_boxes = reader
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<Point>>>()
        .unwrap();
    let mut pairs = all_pairs_no_inverse(&junction_boxes);
    pairs.sort_by(|a, b| {
        straight_line_distance(a.0, a.1).total_cmp(&straight_line_distance(b.0, b.1))
    });
    let circuits: Vec<_> = junction_boxes.iter().map(|&b| HashSet::from([b])).collect();
    (pairs, circuits)
}

fn parse_line(l: Result<String, std::io::Error>) -> Option<(u32, u32, u32)> {
    let l = l.ok()?;
    let mut iter = l.split(',').map(|n| n.parse().ok());
    Some((iter.next()??, iter.next()??, iter.next()??))
}

// with help from: https://stuart.mchattie.net/posts/2025/12/08/aoc-2025-python-day-08/
pub fn part1(path: impl AsRef<Path>, n: usize) -> usize {
    let (pairs, circuits) = parse(path);
    let circuits = pairs.into_iter().take(n).fold(circuits, join_junctions);
    let mut sizes: Vec<_> = circuits.into_iter().map(|c| c.len()).collect();
    sizes.sort_by_key(|&x| Reverse(x));
    sizes.iter().take(3).product()
}

pub fn part2(path: impl AsRef<Path>) -> u32 {
    let (pairs, mut circuits) = parse(path);
    for pair in pairs {
        circuits = join_junctions(circuits, pair);
        if circuits.len() == 1 {
            return pair.0 .0 * pair.1 .0;
        }
    }
    panic!("ran out of values!");
}

fn join_junctions(circuits: Circuits, (p1, p2): (Point, Point)) -> Circuits {
    let (to_merge, mut rest): (Vec<_>, Vec<_>) = circuits
        .into_iter()
        .partition(|c| c.contains(&p1) || c.contains(&p2));
    rest.push(to_merge.into_iter().flatten().collect());
    rest
}

fn sub_point((x1, y1, z1): Point, (x2, y2, z2): Point) -> Point {
    (x1.abs_diff(x2), y1.abs_diff(y2), z1.abs_diff(z2))
}

fn abs((x, y, z): Point) -> f32 {
    ((x as f32).powi(2) + (y as f32).powi(2) + (z as f32).powi(2)).sqrt()
}

fn straight_line_distance(p1: Point, p2: Point) -> f32 {
    abs(sub_point(p1, p2))
}

fn all_pairs_no_inverse<T>(values: &[T]) -> Vec<(T, T)>
where
    T: Copy,
{
    let mut result = vec![];
    let len = values.len();
    for i in 0..len {
        for j in (i + 1)..len {
            result.push((values[i], values[j]));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(40, part1("inputs/practice/8.txt", 10));
    }

    #[test]
    fn part1_real() {
        assert_eq!(135169, part1("inputs/real/8.txt", 1000));
    }

    #[test]
    fn part2_practice() {
        assert_eq!(25272, part2("inputs/practice/8.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(302133440, part2("inputs/real/8.txt"));
    }
}
