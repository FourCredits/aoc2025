use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn part1(path: impl AsRef<Path>) -> usize {
    accessible_rolls(&parse(path)).count()
}

pub fn part2(path: impl AsRef<Path>) -> usize {
    let mut paper_rolls = parse(path);
    let mut total = 0;
    loop {
        let removed: Vec<_> = accessible_rolls(&paper_rolls).cloned().collect();
        if removed.len() == 0 {
            break;
        }
        total += removed.len();
        for r in removed {
            paper_rolls.remove(&r);
        }
    }
    total
}

fn parse(path: impl AsRef<Path>) -> HashSet<(isize, isize)> {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut paper_rolls = HashSet::new();
    for (row, line) in reader.lines().enumerate() {
        for (col, char) in line.unwrap().chars().enumerate() {
            if char == '@' {
                paper_rolls.insert((row as isize, col as isize));
            }
        }
    }
    paper_rolls
}

fn accessible_rolls(
    paper_rolls: &HashSet<(isize, isize)>,
) -> impl Iterator<Item = &(isize, isize)> {
    paper_rolls.iter().filter(|&&pos| {
        neighbours(pos)
            .iter()
            .filter(|neighbour| paper_rolls.contains(neighbour))
            .count()
            < 4
    })
}

fn neighbours((row, col): (isize, isize)) -> [(isize, isize); 8] {
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_practice() {
        assert_eq!(13, part1("inputs/practice/4.txt"))
    }

    #[test]
    fn part_1_real() {
        assert_eq!(1489, part1("inputs/real/4.txt"))
    }

    #[test]
    fn part_2_practice() {
        assert_eq!(43, part2("inputs/practice/4.txt"))
    }

    #[test]
    fn part_2_real() {
        assert_eq!(8890, part2("inputs/real/4.txt"))
    }
}
