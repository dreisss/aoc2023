/// program logic:
/// take each line and filter to numeric only
/// join first and last from this filter
/// sum this value for all lines
#[allow(dead_code)]
pub fn solution(input: &str) -> i32 {
    fn get_calibration_value(line: &str) -> i32 {
        let numbers_only: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();
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
        const TEST_INPUT: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        const EXPECTED: i32 = 142;

        assert!(solution(TEST_INPUT) == EXPECTED);
    }

    #[test]
    fn test_solution_final() {
        const INPUT: &str = include_str!("../../inputs/day01.txt");
        const EXPECTED: i32 = 55002;

        assert!(solution(INPUT) == EXPECTED);
    }
}
