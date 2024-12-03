pub trait Day {
    type Input: Clone;

    fn part1(input: Self::Input) -> String;
    fn part2(input: Self::Input) -> String;
    fn parse_input(input: &[impl AsRef<str>]) -> Self::Input;

    fn run(input: &[impl AsRef<str>]) -> (String, String) {
        let input = Self::parse_input(input);
        (Self::part1(input.clone()), Self::part2(input))
    }
}
