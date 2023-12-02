pub fn solve_part1(input: &str) -> usize {
    input.lines().count()
}

pub fn solve_part2(input: &str) -> usize {
    input.lines().count()
}

pub const EXAMPLE: &str = include_str!("../example.txt");
pub const INPUT: &str = include_str!("../input.txt");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 42);
    }

    #[test]
    #[ignore = "not done yet"]
    fn part1_result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 42);
    }

    #[test]
    #[ignore = "not done yet"]
    fn part2_example() {
        let result = solve_part2(EXAMPLE);
        assert_eq!(result, 42);
    }

    #[test]
    #[ignore = "not done yet"]
    fn part2_result() {
        let result = solve_part2(INPUT);
        assert_eq!(result, 42);
    }
}
