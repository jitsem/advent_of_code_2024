use crate::common::day::Day;
use std::error::Error;

pub struct Day4 {
    pub input: String,
}

impl Day for Day4 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let parser = XmasParser::from(&self.input)?;
        let mut xmas_count = 0;
        for direction in ParseDirection::VALUES {
            for i in 0..parser.len() {
                if parser.is_xmas(i, &direction) {
                    xmas_count += 1;
                }
            }
        }

        Ok(xmas_count.to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let parser = XmasParser::from(&self.input)?;
        let mut xmas_count = 0;
        for i in 0..parser.len() {
            if parser.is_cross_mas(i) {
                xmas_count += 1;
            }
        }
        Ok(xmas_count.to_string())
    }
}

enum ParseDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    LeftTopToRightBottom,
    RightTopToLeftBottom,
    LeftBottomToRightTop,
    RightBottomToLeftTop,
}
impl ParseDirection {
    const VALUES: [Self; 8] = [
        Self::LeftToRight,
        Self::RightToLeft,
        Self::TopToBottom,
        Self::BottomToTop,
        Self::LeftTopToRightBottom,
        Self::RightTopToLeftBottom,
        Self::LeftBottomToRightTop,
        Self::RightBottomToLeftTop,
    ];
}

struct XmasParser {
    input: Vec<char>,
    width: i32,
}

impl XmasParser {
    fn from(input: &str) -> Result<Self, Box<dyn Error>> {
        let width = input
            .lines()
            .next()
            .ok_or::<String>("Received empty input".into())?
            .len();
        let input: Vec<char> = input.chars().filter(|c| !c.is_ascii_whitespace()).collect();
        Ok(XmasParser {
            input,
            width: width as i32,
        })
    }

    fn len(&self) -> usize {
        self.input.len()
    }

    fn is_cross_mas(&self, pos: usize) -> bool {
        if self.input[pos] != 'A' {
            return false;
        }
        let pos = pos as i32;
        if (pos % self.width) - 1 < 0 || (pos % self.width) + 1 >= self.width {
            return false;
        }
        let cross = (
            pos - self.width - 1,
            pos - self.width + 1,
            pos + self.width - 1,
            pos + self.width + 1,
        );
        matches!(
            self.get_chars_2(cross),
            (Some('M'), Some('M'), Some('S'), Some('S'))
                | (Some('M'), Some('S'), Some('M'), Some('S'))
                | (Some('S'), Some('S'), Some('M'), Some('M'))
                | (Some('S'), Some('M'), Some('S'), Some('M'))
        )
    }

    fn is_xmas(&self, pos: usize, direction: &ParseDirection) -> bool {
        if self.input[pos] != 'X' {
            return false;
        }
        let pos = pos as i32;
        let maybe_mas = match direction {
            ParseDirection::LeftToRight if (pos % self.width) + 3 <= self.width - 1 => {
                (pos + 1, pos + 2, pos + 3)
            }
            ParseDirection::RightToLeft if (pos % self.width) - 3 >= 0 => {
                (pos - 1, pos - 2, pos - 3)
            }
            ParseDirection::TopToBottom => (
                pos + self.width,
                pos + (2 * self.width),
                pos + (3 * self.width),
            ),
            ParseDirection::BottomToTop => (
                pos - self.width,
                pos - (2 * self.width),
                pos - (3 * self.width),
            ),
            ParseDirection::LeftTopToRightBottom if (pos % self.width) + 3 <= self.width - 1 => (
                pos + self.width + 1,
                pos + (2 * self.width) + 2,
                pos + (3 * self.width) + 3,
            ),
            ParseDirection::RightTopToLeftBottom if (pos % self.width) - 3 >= 0 => (
                pos + self.width - 1,
                pos + (2 * self.width) - 2,
                pos + (3 * self.width) - 3,
            ),
            ParseDirection::LeftBottomToRightTop if (pos % self.width) + 3 <= self.width - 1 => (
                pos - self.width + 1,
                pos - (2 * self.width) + 2,
                pos - (3 * self.width) + 3,
            ),
            ParseDirection::RightBottomToLeftTop if (pos % self.width) - 3 >= 0 => (
                pos - self.width - 1,
                pos - (2 * self.width) - 2,
                pos - (3 * self.width) - 3,
            ),
            _ => return false,
        };

        match self.get_chars(maybe_mas) {
            (Some('M'), Some('A'), Some('S')) => true,
            (_, _, _) => false,
        }
    }
    fn get_chars(&self, train: (i32, i32, i32)) -> (Option<&char>, Option<&char>, Option<&char>) {
        if train.0.is_negative() || train.1.is_negative() || train.2.is_negative() {
            return (None, None, None);
        }
        (
            self.input.get(train.0 as usize),
            self.input.get(train.1 as usize),
            self.input.get(train.2 as usize),
        )
    }

    fn get_chars_2(
        &self,
        train: (i32, i32, i32, i32),
    ) -> (Option<&char>, Option<&char>, Option<&char>, Option<&char>) {
        if train.0.is_negative()
            || train.1.is_negative()
            || train.2.is_negative()
            || train.3.is_negative()
        {
            return (None, None, None, None);
        }
        (
            self.input.get(train.0 as usize),
            self.input.get(train.1 as usize),
            self.input.get(train.2 as usize),
            self.input.get(train.3 as usize),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let day = Day4 {
            input: "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                .to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "18");
    }
    #[test]
    fn part2_example() {
        let day = Day4 {
            input: "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
                .to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "9");
    }
}
