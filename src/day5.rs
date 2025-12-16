use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    path::Path,
};

pub fn part1(path: impl AsRef<Path>) -> usize {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut iter = reader.lines().map(|l| l.unwrap());
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        let (lo, hi) = line.split_once('-').unwrap();
        ranges.push(lo.parse().unwrap()..=hi.parse().unwrap());
    }
    let mut fresh_count = 0;
    for line in iter {
        let line = line.parse().unwrap();
        if ranges.iter().any(move |range| range.contains(&line)) {
            fresh_count += 1;
        }
    }
    fresh_count
}

// got some help from here: https://github.com/anup30/advent-of-code/blob/main/advent%20of%20code/2025/day%205%20part%202.cpp
pub fn part2(path: impl AsRef<Path>) -> usize {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut iter = reader.lines().map(|l| l.unwrap());
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        let (lo, hi) = line.split_once('-').unwrap();
        ranges.push((lo.parse().unwrap(), hi.parse().unwrap()));
    }
    ranges.sort();
    let mut merged: Vec<(usize, usize)> = vec![ranges[0]];
    for r in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();
        if r.0 <= last.1 {
            last.1 = last.1.max(r.1);
        } else {
            merged.push(r);
        }
    }
    merged.into_iter().map(|(lo, hi)| hi - lo + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(3, part1("inputs/practice/5.txt"));
    }

    #[test]
    fn part1_real() {
        assert_eq!(640, part1("inputs/real/5.txt"));
    }

    #[test]
    fn part2_practice() {
        assert_eq!(14, part2("inputs/practice/5.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(365804144481581, part2("inputs/real/5.txt"));
    }
}
