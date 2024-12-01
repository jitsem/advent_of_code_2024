pub trait Day {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn part2(&self) -> Result<String, Box<dyn std::error::Error>>;
}
