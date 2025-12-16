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
    todo!()
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
}
