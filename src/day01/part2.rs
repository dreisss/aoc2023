const SPELLED_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// program logic:
/// take each line, transform each spelled number into digits and filter to numeric only
/// join first and last from this filter
/// sum this value for all lines
#[allow(dead_code)]
pub fn solution(input: &str) -> i32 {
    fn get_calibration_value(line: &str) -> i32 {
        let mut parsed_line = line.to_string();

        for (i, number) in SPELLED_NUMBERS.iter().enumerate() {
            parsed_line = parsed_line.replace(number, &format!("{}{}{}", number, i + 1, number));
        }

        let numbers_only: Vec<char> = parsed_line.chars().filter(|c| c.is_numeric()).collect();
        let mut result = String::new();

        result.push(*numbers_only.first().unwrap());
        result.push(*numbers_only.last().unwrap());

        result.parse().unwrap()
    }

    input.lines().map(|l| get_calibration_value(l)).sum()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_solution() {
        const TEST_INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        const EXPECTED: i32 = 281;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day01.txt");
        const EXPECTED: i32 = 55093;

        assert!(solution(INPUT) == EXPECTED);
    }
}
