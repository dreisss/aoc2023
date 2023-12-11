use std::collections::{HashMap, HashSet};

/// program logic:
#[allow(dead_code)]
fn solution(input: &str) -> i32 {
    fn get_matching_numbers(line: &str) -> usize {
        let mut numbers = line.split(':').last().unwrap().split('|');

        let winning_numbers: HashSet<&str> = numbers.next().unwrap().split_whitespace().collect();
        let my_numbers: HashSet<&str> = numbers.last().unwrap().split_whitespace().collect();
        let matching_numbers: HashSet<&&str> = my_numbers.intersection(&winning_numbers).collect();

        matching_numbers.len()
    }

    let mut scratchcards = HashMap::new();

    for i in 0..input.lines().count() {
        scratchcards.insert(i, 1);
    }

    for (i, line) in input.lines().enumerate() {
        for c_i in 1..get_matching_numbers(line) + 1 {
            scratchcards.insert(
                i + c_i,
                scratchcards.get(&(i + c_i)).unwrap() + scratchcards.get(&i).unwrap(),
            );
        }
    }

    scratchcards.values().sum()
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::solution;

    #[test]
    fn test_solution() {
        const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        const EXPECTED: i32 = 30;

        assert_eq!(solution(TEST_INPUT), EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day04.txt");
        const EXPECTED: i32 = 9496801;

        assert_eq!(solution(INPUT), EXPECTED);
    }
}
