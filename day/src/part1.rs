pub fn solve_part1(input: &str) -> usize {
    input.lines().count()
}

#[cfg(test)]
mod tests {
    use super::super::INPUT;
    use super::*;

    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn example() {
        let result = solve_part1(EXAMPLE);
        assert_eq!(result, 42);
    }

    #[test]
    #[ignore = "not done yet"]
    fn result() {
        let result = solve_part1(INPUT);
        assert_eq!(result, 42);
    }
}
