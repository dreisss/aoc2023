use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: i32,
    x: usize,
    y: usize,
    length: usize,
}

/// program logic:
/// take each line and get every number with its value, cordinate and length
/// take each symbol and get numbers around it
/// sum this numbers
#[allow(dead_code)]
pub fn solution(input: &str) -> i32 {
    fn is_around_symbol(x: usize, y: usize, number: &Number) -> bool {
        if number.y + 1 >= y && number.y <= y + 1 {
            let lx = number.x;
            let rx = number.x + number.length - 1;

            for number_x in lx..=rx {
                if number_x + 1 >= x && number_x <= x + 1 {
                    return true;
                }
            }
        }

        false
    }

    let mut all_numbers: HashSet<Number> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let mut number = String::new();

        for (x, char) in line.chars().enumerate() {
            if char.is_numeric() {
                number.push(char);
            }

            if !number.is_empty() && (!char.is_numeric() || x == line.len() - 1) {
                all_numbers.insert(Number {
                    value: number.parse().unwrap(),
                    x: x.wrapping_sub(number.len()),
                    y,
                    length: number.len(),
                });

                number = String::new();
            }
        }
    }

    let mut part_numbers: HashSet<&Number> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' && !char.is_numeric() {
                for number in all_numbers.iter() {
                    if is_around_symbol(x, y, number) {
                        part_numbers.insert(number);
                    }
                }
            }
        }
    }

    part_numbers.into_iter().map(|n| n.value).sum()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_edge_cases() {
        const TEST_INPUT: &str = "12.......*..\n+.........34\n.......-12..\n..78........\n..*....60...\n78.........9\n.5.....23..$\n8...90*12...\n............\n2.2......12.\n.*.........*\n1.1..503+.56";
        const EXPECTED: i32 = 925;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution() {
        const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        const EXPECTED: i32 = 4361;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day03.txt");
        const EXPECTED: i32 = 531932;

        assert!(solution(INPUT) == EXPECTED);
    }
}
