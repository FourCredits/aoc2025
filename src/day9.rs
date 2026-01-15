use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Point = (usize, usize);

fn parse(path: impl AsRef<Path>) -> Vec<(usize, usize)> {
    let reader = BufReader::new(File::open(path).unwrap());
    let points = reader
        .lines()
        .map(parse_line)
        .collect::<Option<Vec<_>>>()
        .unwrap();
    points
}

pub fn part1(path: impl AsRef<Path>) -> usize {
    let pairs = all_pairs_no_inverse(&parse(path));
    pairs.into_iter().map(area).max().unwrap()
}

fn area((p1, p2): (Point, Point)) -> usize {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn parse_line(l: Result<String, std::io::Error>) -> Option<Point> {
    let l = l.ok()?;
    let mut iter = l.split(',').map(|n| n.parse().ok());
    Some((iter.next()??, iter.next()??))
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
        assert_eq!(50, part1("inputs/practice/9.txt"));
    }

    #[test]
    fn part1_real() {
        assert_eq!(4781377701, part1("inputs/real/9.txt"));
    }
}
