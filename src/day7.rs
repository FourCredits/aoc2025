use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn parse(path: impl AsRef<Path>) -> (usize, impl Iterator<Item = String>) {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut lines = reader.lines().map(|l| l.unwrap());
    let first_line = lines.next().unwrap();
    let starting_beam = first_line.find('S').unwrap();
    (starting_beam, lines)
}

pub fn part1(path: impl AsRef<Path>) -> usize {
    let (starting_beam, lines) = parse(path);
    let mut beam_columns = HashSet::from([starting_beam]);
    let mut times_split = 0;
    for line in lines {
        let line = line.as_bytes();
        let mut new_beams = HashSet::new();
        for &beam in beam_columns.iter() {
            if line[beam] == b'^' {
                times_split += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            } else {
                new_beams.insert(beam);
            }
        }
        beam_columns = new_beams;
    }
    times_split
}

pub fn part2(path: impl AsRef<Path>) -> usize {
    let (starting_beam, lines) = parse(path);
    let mut beam_columns = HashMap::from([(starting_beam, 1)]);
    for line in lines {
        let line = line.as_bytes();
        let mut new_beams = HashMap::new();
        for (&beam, &num_universes) in beam_columns.iter() {
            if line[beam] == b'^' {
                *new_beams.entry(beam - 1).or_insert(0) += num_universes;
                *new_beams.entry(beam + 1).or_insert(0) += num_universes;
            } else {
                *new_beams.entry(beam).or_insert(0) += num_universes;
            }
        }
        beam_columns = new_beams;
    }
    beam_columns.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_practice() {
        assert_eq!(21, part1("inputs/practice/7.txt"));
    }

    #[test]
    fn part1_real() {
        assert_eq!(1524, part1("inputs/real/7.txt"));
    }

    #[test]
    fn part2_practice() {
        assert_eq!(40, part2("inputs/practice/7.txt"));
    }

    #[test]
    fn part2_real() {
        assert_eq!(32982105837605, part2("inputs/real/7.txt"));
    }
}
