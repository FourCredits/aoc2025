use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn max_joltage_n(n: usize, batteries: &[usize]) -> usize {
    assert!(n <= batteries.len());
    let mut batteries = batteries;
    let mut total = 0;
    for i in (1..=n).rev() {
        let (index, biggest_digit) = (&batteries[..batteries.len() - i + 1])
            .iter()
            .enumerate()
            .reduce(|prev, next| cmp::max_by_key(next, prev, |x| x.1))
            .unwrap();
        total = total * 10 + biggest_digit;
        batteries = &batteries[index + 1..];
    }
    total
}

fn parse_line(s: &str) -> Vec<usize> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn solve(num_active_batteries: usize, path: impl AsRef<Path>) -> usize {
    BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|bank_string| max_joltage_n(num_active_batteries, &parse_line(&bank_string.unwrap())))
        .sum()
}

pub fn part1(path: impl AsRef<Path>) -> usize {
    solve(2, path)
}

pub fn part2(path: impl AsRef<Path>) -> usize {
    solve(12, path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(357, part1("inputs/practice/3.txt"));
    }

    #[test]
    fn part1_real() {
        assert_eq!(16854, part1("inputs/real/3.txt"));
    }

    #[test]
    fn part2_practice() {
        assert_eq!(3121910778619, part2("inputs/practice/3.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(167526011932478, part2("inputs/real/3.txt"));
    }
}
