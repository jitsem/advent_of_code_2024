use crate::common::day::Day;
use std::collections::HashMap;
use std::num::ParseIntError;
pub struct Day1 {
    pub input: String,
}

impl Day for Day1 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (left, right) = self.get_left_right();

        let mut left: Vec<i32> = left.collect::<Result<Vec<_>, _>>()?;
        let mut right: Vec<i32> = right.collect::<Result<Vec<_>, _>>()?;
        left.sort();
        right.sort();

        Ok(left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>()
            .to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (left, right) = self.get_left_right();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for r in right {
            map.entry(r?).and_modify(|v| *v += 1).or_insert(1);
        }
        let folded: Result<i32, ParseIntError> = left.into_iter().try_fold(0, |acc, l| {
            let mapped_value = l.map(|x| map.get(&x).unwrap_or(&0) * x)?;
            Ok(acc + mapped_value)
        });

        Ok(folded?.to_string())
    }
}

impl Day1 {
    fn get_left_right(
        &self,
    ) -> (
        impl Iterator<Item = Result<i32, ParseIntError>> + '_,
        impl Iterator<Item = Result<i32, ParseIntError>> + '_,
    ) {
        let left = self
            .input
            .split_whitespace()
            .step_by(2)
            .map(|s| s.parse::<i32>());
        let right = self
            .input
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<i32>());
        (left, right)
    }
}

#[cfg(test)]
mod tests {
    use crate::common::day::Day;
    use crate::days::day1::Day1;

    #[test]
    fn part1_example() {
        let day = Day1 {
            input: "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "11");
    }
    #[test]
    fn part2_example() {
        let day = Day1 {
            input: "3   4\n4   3\n2   5\n1   3\n3   9\n3   3".to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "31");
    }
}
