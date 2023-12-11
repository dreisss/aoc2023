use std::collections::HashSet;

/// program logic:
/// for each line get winning and my numbers
/// calculate how many are in common
/// get points for each line
/// sum points for each line
#[allow(dead_code)]
fn solution(input: &str) -> i32 {
    fn get_points(line: &str) -> i32 {
        let mut numbers = line.split(':').last().unwrap().split('|');

        let winning_numbers: HashSet<&str> = numbers.next().unwrap().split_whitespace().collect();
        let my_numbers: HashSet<&str> = numbers.last().unwrap().split_whitespace().collect();
        let matching_numbers: HashSet<&&str> = my_numbers.intersection(&winning_numbers).collect();

        if matching_numbers.len() == 0 {
            return 0;
        }

        i32::pow(2, matching_numbers.len() as u32 - 1)
    }

    input.lines().map(|l| get_points(l)).sum()
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
        const EXPECTED: i32 = 13;

        assert_eq!(solution(TEST_INPUT), EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day04.txt");
        const EXPECTED: i32 = 27845;

        assert_eq!(solution(INPUT), EXPECTED);
    }
}
