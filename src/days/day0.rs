use crate::common::day::Day;

pub struct Day0 {
    pub input: String,
}

/// This is just a testing day, to make sure the 'framework' works
impl Day for Day0 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        //Get the even chars from string and concat
        Ok(self
            .input
            .trim()
            .chars()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 2 == 0)
            .map(|(_, ch)| ch)
            .collect::<String>())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        //Get the even chars from string and concat, if number add to the result and return string
        Ok(self
            .input
            .trim()
            .chars()
            .enumerate()
            .filter(|(i, _)| (i + 1) % 2 == 0)
            .map(|(_, ch)| ch.to_digit(10).unwrap_or(0))
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::common::day::Day;
    use crate::days::day0::Day0;

    #[test]
    fn part1_example() {
        let day = Day0 {
            input: "A1B2C3DEFG".to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "123EG");
    }
    #[test]
    fn part2_example() {
        let day = Day0 {
            input: "A1B2C3DEFG".to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "6");
    }
}
