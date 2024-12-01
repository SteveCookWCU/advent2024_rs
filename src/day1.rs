pub fn part1(input: &[impl AsRef<str>]) -> String {
    String::new()
}

pub fn part2(input: &[impl AsRef<str>]) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};
    use crate::get_test_input;

    static INPUT: &'static str = "";

    #[test]
    fn test_part1() {
        let input = get_test_input(INPUT);
        assert_eq!(part1(&input), "");
    }

    #[test]
    fn test_part2() {
        let input = get_test_input(INPUT);
        assert_eq!(part2(&input), "");
    }
}
