use crate::common::day::Day;
use std::collections::HashMap;
pub struct Day5 {
    pub input: String,
}

impl Day for Day5 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut split = self.input.split("\n\n");
        let mut sum = 0;
        let rule_engine = RuleEngine::from(split.next().ok_or("Did not find filter list")?)?;
        for update_line in self
            .get_updates(split.next().ok_or("Did not find update list")?)?
            .iter()
        {
            if rule_engine.is_valid(update_line) {
                sum += update_line[(update_line.len() - 1) / 2];
            }
        }

        Ok(sum.to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut split = self.input.split("\n\n");
        let mut sum = 0;
        let rule_engine = RuleEngine::from(split.next().ok_or("Did not find filter list")?)?;
        for update_line in self
            .get_updates(split.next().ok_or("Did not find update list")?)?
            .iter_mut()
        {
            if !rule_engine.is_valid(update_line) {
                rule_engine.apply(update_line);
                sum += update_line[(update_line.len() - 1) / 2];
            }
        }

        Ok(sum.to_string())
    }
}

impl Day5 {
    fn get_updates(&self, raw_updates: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
        let lines = raw_updates
            .lines()
            .map(|l| {
                let update_line = l
                    .trim()
                    .split(',')
                    .map(|s| {
                        let update = s.parse::<i32>()?;
                        Ok::<_, Box<dyn std::error::Error>>(update)
                    })
                    .collect::<Result<Vec<i32>, _>>()?;
                Ok::<_, Box<dyn std::error::Error>>(update_line)
            })
            .collect::<Result<Vec<Vec<i32>>, _>>();
        lines
    }
}
struct RuleEngine {
    rules: HashMap<i32, Vec<i32>>,
}

impl RuleEngine {
    fn from(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parsed_rules = input
            .lines()
            .map(|l| {
                let mut split = l.trim().split('|');
                let split1 = split
                    .next()
                    .ok_or::<String>("Encountered wrong rule input".into())?
                    .parse::<i32>()?;
                let split2 = split
                    .next()
                    .ok_or::<String>("Encountered wrong rule input".into())?
                    .parse::<i32>()?;

                Ok::<(i32, i32), Box<dyn std::error::Error>>((split1, split2))
            })
            .collect::<Result<Vec<(i32, i32)>, _>>()?;

        let mut rules = HashMap::new();
        for rule in parsed_rules {
            rules.entry(rule.0).or_insert(Vec::new()).push(rule.1);
        }

        Ok(RuleEngine { rules })
    }

    fn is_valid(&self, input: &[i32]) -> bool {
        let mut buffer: Vec<i32> = Vec::with_capacity(input.len());
        for el in input.iter() {
            if let Some(entry) = self.rules.get(el) {
                let matches = entry.iter().filter(|e| !buffer.iter().all(|b| b != *e));
                if matches.count() > 0 {
                    return false;
                }
            }
            buffer.push(*el);
        }
        true
    }

    fn apply(&self, input: &mut [i32]) {
        input.sort_by(|a, b| {
            if let Some(rules) = self.rules.get(a) {
                if rules.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }
            if let Some(rules) = self.rules.get(b) {
                if rules.contains(a) {
                    return std::cmp::Ordering::Greater;
                }
            }

            std::cmp::Ordering::Equal
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let day = Day5 {
            input: "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47"
                .to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "143");
    }
    #[test]
    fn part2_example() {
        let day = Day5 {
            input: "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47"
                .to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "123");
    }
}
