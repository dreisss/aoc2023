const SPELLED_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// program logic:
/// take each line, transform each spelled number into digits and filter to numeric only
/// join first and last from this filter
/// sum this value for all lines
pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> u32 {
            let mut parsed_line = line.to_string();

            for (i, number) in SPELLED_NUMBERS.iter().enumerate() {
                parsed_line = parsed_line.replace(
                    number,
                    &format!("{}{}{}", number, (i + 1).to_string(), number),
                );
            }

            let filtered_line: Vec<char> = parsed_line.chars().filter(|c| c.is_numeric()).collect();
            let mut result = String::new();

            result.push(*filtered_line.first().unwrap());
            result.push(*filtered_line.last().unwrap());

            result.parse().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::solution;

    #[test]
    fn test_solution() {
        const TEST_INPUT: &str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        const EXPECTED: u32 = 281;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/01/b.txt");
        const EXPECTED: u32 = 55093;

        assert!(solution(INPUT) == EXPECTED);
    }
}
