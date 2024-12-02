use crate::common::day::Day;
use std::num::ParseIntError;
pub struct Day2 {
    pub input: String,
}

impl Day for Day2 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut safety_score = 0;
        for numbers in self.create_lines_as_vec_iter() {
            let numbers = numbers?;
            if check_vector_is_ok(&numbers)? {
                safety_score += 1
            }
        }
        Ok(safety_score.to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut safety_score = 0;
        for numbers in self.create_lines_as_vec_iter() {
            let numbers = numbers?;
            let mut dampened: Vec<i32> = Vec::with_capacity(numbers.len() - 1);
            for to_remove in 0..numbers.len() {
                dampened.clear();
                dampened.extend(numbers.iter().enumerate().filter_map(|(i, el)| {
                    if i == to_remove {
                        None
                    } else {
                        Some(el)
                    }
                }));

                if check_vector_is_ok(&dampened)? {
                    safety_score += 1;
                    break;
                }
            }
        }
        Ok(safety_score.to_string())
    }
}

impl Day2 {
    fn create_lines_as_vec_iter(
        &self,
    ) -> impl Iterator<Item = Result<Vec<i32>, ParseIntError>> + '_ {
        self.input.lines().map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
        })
    }
}

fn check_vector_is_ok(numbers: &[i32]) -> Result<bool, Box<dyn std::error::Error>> {
    if numbers.len() < 2 {
        //Always true
        return Ok(true);
    }
    let ascending = (numbers[0] - numbers[1]).is_negative();
    for el in 1..numbers.len() {
        let diff = numbers[el - 1] - numbers[el];
        if diff.abs() > 3 || diff.abs() < 1 {
            return Ok(false);
        }
        match ascending {
            true if diff.is_negative() => {}
            false if diff.is_positive() => {}
            _ => return Ok(false),
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let day = Day2 {
            input: "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"
            .to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "2");
    }
    #[test]
    fn part2_example() {
        let day = Day2 {
            input: "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
                .to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "4");
    }
}
