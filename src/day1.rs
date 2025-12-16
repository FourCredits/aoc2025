use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_file(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(path)?).lines().collect()
}

pub fn parse(line: impl AsRef<str>) -> i32 {
    let line = line.as_ref();
    let direction = match line.chars().next().unwrap() {
        'L' => -1,
        'R' => 1,
        _ => panic!("bad input format"),
    };
    direction * ((&line[1..]).parse::<i32>().unwrap())
}

pub fn process_part1(values: impl Iterator<Item = i32>) -> i32 {
    values
        .fold((50, 0), |(position, count), value| {
            let new_position = (position + value).rem_euclid(100);
            let new_count = if new_position == 0 { count + 1 } else { count };
            (new_position, new_count)
        })
        .1
}

pub fn part1(path: impl AsRef<Path>) -> i32 {
    process_part1(read_file(path).unwrap().iter().map(parse))
}

pub fn process_part2(values: impl Iterator<Item = i32>) -> i32 {
    let mut position = 50;
    let mut times_zero_crossed = 0;
    for value in values {
        // it ain't pretty, but trying to be smart didn't work
        for n in (0..(value.abs())).map(|_| value.signum()) {
            position = (position + n).rem_euclid(100);
            if position == 0 {
                times_zero_crossed += 1;
            }
        }
    }
    times_zero_crossed
}

pub fn part2(path: impl AsRef<Path>) -> i32 {
    process_part2(read_file(path).unwrap().iter().map(parse))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn practice_part1() {
        assert_eq!(3, part1("inputs/practice/1.txt"));
    }

    #[test]
    fn real_part1() {
        assert_eq!(1074, part1("inputs/real/1.txt"));
    }

    #[test]
    fn practice_part2() {
        assert_eq!(6, part2("inputs/practice/1.txt"));
    }

    #[test]
    fn real_part2() {
        assert_eq!(6254, part2("inputs/real/1.txt"));
    }
}
