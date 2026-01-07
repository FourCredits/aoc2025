use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn part1(path: impl AsRef<Path>) -> usize {
    let reader = BufReader::new(File::open(path).unwrap());
    let lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();
    let mut state: Vec<(usize, fn(usize, usize) -> usize)> = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| make_initial_state(&s))
        .collect();
    let len = state.len();
    for line in &lines[..lines.len() - 1] {
        let nums = line
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        for i in 0..len {
            let (value, op) = state[i];
            state[i].0 = op(value, nums[i]);
        }
    }
    state.into_iter().map(|pair| pair.0).sum()
}

fn make_initial_state(s: &str) -> (usize, fn(usize, usize) -> usize) {
    match s {
        "*" => (1, |a, b| a * b),
        "+" => (0, |a, b| a + b),
        _ => panic!("oops!"),
    }
}

pub fn part2(path: impl AsRef<Path>) -> usize {
    let reader = BufReader::new(File::open(path).unwrap());
    let chars: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let columns = chars[0].len();
    let rows = chars.len();
    let mut state: Option<(fn(usize, usize) -> usize, usize)> = None;
    let add = |a, b| a + b;
    let mul = |a, b| a * b;
    let mut grand_total = 0;
    for c in 0..columns {
        let number_string: String = (&chars[..rows - 1]).iter().map(|r| r[c]).collect();
        let trimmed = number_string.trim();
        if trimmed.is_empty() {
            if let Some((_, acc)) = state {
                grand_total += acc;
            }
            state = None;
            continue;
        }
        let number: usize = trimmed.parse().unwrap();
        state = match chars[rows - 1][c] {
            '+' => Some((add, 0)),
            '*' => Some((mul, 1)),
            _ => state,
        };
        match state {
            None => panic!("whoops! there's a bug here"),
            Some((op, acc)) => {
                state = Some((op, op(acc, number)));
            }
        }
    }
    if let Some((_, acc)) = state {
        grand_total += acc;
    }
    grand_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(4277556, part1("inputs/practice/6.txt"));
    }

    #[test]
    fn part1_real() {
        assert_eq!(4405895212738, part1("inputs/real/6.txt"));
    }

    #[test]
    fn part2_practice() {
        assert_eq!(3263827, part2("inputs/practice/6.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(7450962489289, part2("inputs/real/6.txt"));
    }
}
