/// program logic:
/// take each line and filter to numeric only
/// join first and last from this filter
/// sum this value for all lines
pub fn solution(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> u32 {
            let filtered_line: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
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
        const TEST_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        const EXPECTED: u32 = 142;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/01/a.txt");
        const EXPECTED: u32 = 55002;

        assert!(solution(INPUT) == EXPECTED);
    }
}
