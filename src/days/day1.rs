use crate::common::day::Day;
use std::collections::HashMap;
use std::num::ParseIntError;
pub struct Day1 {
    pub input: String,
}

impl Day for Day1 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let (mut left, mut right) = self.get_left_right()?;

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
        let (left, right) = self.get_left_right()?;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for r in right {
            map.entry(r).and_modify(|v| *v += 1).or_insert(1);
        }
        Ok(left
            .iter()
            .map(|l| map.get(l).unwrap_or(&0) * l)
            .sum::<i32>()
            .to_string())
    }
}

impl Day1 {
    fn get_left_right(&self) -> Result<(Vec<i32>, Vec<i32>), ParseIntError> {
        let left: Result<Vec<i32>, _> = self
            .input
            .split_whitespace()
            .step_by(2)
            .map(|s| s.parse::<i32>())
            .collect();
        let right: Result<Vec<i32>, _> = self
            .input
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|s| s.parse::<i32>())
            .collect();
        Ok((left?, right?))
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
