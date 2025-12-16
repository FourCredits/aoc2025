use std::path::Path;

pub fn get_input(path: impl AsRef<Path>) -> Vec<usize> {
    let input = std::fs::read(path).unwrap();
    let input = str::from_utf8(&input).unwrap();
    input
        .trim()
        .split(',')
        .flat_map(|range_string| {
            let (lo, hi) = range_string.split_once('-').unwrap();
            lo.parse().unwrap()..=hi.parse().unwrap()
        })
        .collect()
}

pub fn part1(path: impl AsRef<Path>) -> usize {
    get_input(path)
        .into_iter()
        .filter(|n| {
            let number_string = n.to_string();
            let middle = number_string.len() / 2;
            &number_string[..middle] == &number_string[middle..]
        })
        .sum()
}

pub fn part2(path: impl AsRef<Path>) -> usize {
    get_input(path)
        .into_iter()
        .filter(|n| {
            let number_string = n.to_string();
            let total_length = number_string.len();
            (0..=total_length / 2).any(|substring_length| {
                total_length.is_multiple_of(substring_length)
                    && number_string
                        .trim_start_matches(&number_string[..substring_length])
                        .is_empty()
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(1227775554, part1("inputs/practice/2.txt"))
    }

    #[test]
    fn part1_real() {
        assert_eq!(40398804950, part1("inputs/real/2.txt"))
    }

    #[test]
    fn part2_practice() {
        assert_eq!(4174379265, part2("inputs/practice/2.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(65794984339, part2("inputs/real/2.txt"));
    }
}
