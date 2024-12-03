use crate::common::day::Day;
use crate::days::day3::FunctionType::{Dont, Multiply};
use crate::days::day3::Token::LiteralNumer;

pub struct Day3 {
    pub input: String,
}

impl Day for Day3 {
    fn part1(&self) -> Result<String, Box<dyn std::error::Error>> {
        let tokens: Vec<Token> = Tokenizer::from(&self.input).collect();
        let mut sum = 0;
        for expr in Parser::from(tokens) {
            if let Expression::Multiply(x, y) = expr {
                sum += x * y
            }
        }
        Ok(sum.to_string())
    }

    fn part2(&self) -> Result<String, Box<dyn std::error::Error>> {
        let tokens: Vec<Token> = Tokenizer::from(&self.input).collect();
        let mut sum = 0;
        let mut enabled = true;
        for expr in Parser::from(tokens) {
            match expr {
                Expression::Multiply(x, y) if enabled => sum += x * y,
                Expression::Enable(enable) => enabled = enable,
                _ => {}
            }
        }
        Ok(sum.to_string())
    }
}

enum Expression {
    Multiply(i32, i32),
    Enable(bool),
}

struct Parser {
    input: Vec<Token>,
    current: usize,
}

impl Iterator for Parser {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_expression()
    }
}

impl Parser {
    fn from(input: Vec<Token>) -> Self {
        Parser { input, current: 0 }
    }

    fn next_expression(&mut self) -> Option<Expression> {
        if self.is_done() {
            return None;
        }
        while let Some(token) = self.current_token() {
            if *token == Token::Func(Multiply) {
                if let Some(expr) = self.parse_multiply() {
                    return Some(expr);
                }
            } else if *token == Token::Func(FunctionType::Do) {
                if let Some(expr) = self.parse_do() {
                    return Some(expr);
                }
            } else if *token == Token::Func(FunctionType::Dont) {
                if let Some(expr) = self.parse_dont() {
                    return Some(expr);
                }
            }
            self.advance_token(1)
        }
        None
    }

    fn parse_multiply(&mut self) -> Option<Expression> {
        if let (
            Some(&Token::Func(Multiply)),
            Some(&Token::OpenBrace),
            Some(&LiteralNumer(x)),
            Some(&Token::Comma),
            Some(&LiteralNumer(y)),
            Some(&Token::CloseBrace),
        ) = (
            self.current_token(),
            self.peek(1),
            self.peek(2),
            self.peek(3),
            self.peek(4),
            self.peek(5),
        ) {
            self.advance_token(6);
            Some(Expression::Multiply(x, y))
        } else {
            None
        }
    }

    fn parse_do(&mut self) -> Option<Expression> {
        if let (
            Some(&Token::Func(FunctionType::Do)),
            Some(&Token::OpenBrace),
            Some(&Token::CloseBrace),
        ) = (self.current_token(), self.peek(1), self.peek(2))
        {
            self.advance_token(3);
            Some(Expression::Enable(true))
        } else {
            None
        }
    }

    fn parse_dont(&mut self) -> Option<Expression> {
        if let (
            Some(&Token::Func(FunctionType::Dont)),
            Some(&Token::OpenBrace),
            Some(&Token::CloseBrace),
        ) = (self.current_token(), self.peek(1), self.peek(2))
        {
            self.advance_token(3);
            Some(Expression::Enable(false))
        } else {
            None
        }
    }

    fn current_token(&self) -> Option<&Token> {
        if self.is_done() {
            return None;
        }
        self.input.get(self.current)
    }
    fn advance_token(&mut self, step: usize) {
        self.current += step;
    }

    fn peek(&self, step: usize) -> Option<&Token> {
        self.input.get(self.current + step)
    }

    fn is_done(&self) -> bool {
        self.current >= self.input.len()
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Token {
    Eof,
    Func(FunctionType),
    OpenBrace,
    CloseBrace,
    LiteralNumer(i32),
    Comma,
    Unknown,
    Error(String),
}

#[derive(Eq, PartialEq, Debug)]
enum FunctionType {
    Multiply,
    Do,
    Dont,
}

struct Tokenizer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token::Eof) => None,
            Ok(t) => Some(t),
            Err(e) => Some(Token::Error(e.to_string())),
        }
    }
}

impl<'a> Tokenizer<'a> {
    fn from(input: &'a str) -> Self {
        Tokenizer {
            input,
            current_pos: 0,
        }
    }
    fn next_token(&mut self) -> Result<Token, Box<dyn std::error::Error>> {
        if self.is_done() {
            return Ok(Token::Eof);
        }
        let (token, processed_chars) = match self.current_char() {
            None => Ok((Token::Eof, 1)),
            Some('(') => Ok((Token::OpenBrace, 1)),
            Some(')') => Ok((Token::CloseBrace, 1)),
            Some(',') => Ok((Token::Comma, 1)),
            Some(c) if c.is_ascii_digit() => self.try_number(),
            Some(c) if c.is_ascii_alphabetic() => self.try_func(),
            Some(_) => Ok((Token::Unknown, 1)),
        }?;
        self.advance_char(processed_chars);
        Ok(token)
    }

    fn try_number(&mut self) -> Result<(Token, usize), Box<dyn std::error::Error>> {
        let mut number_str = String::new();
        let mut found_chars = 0;
        while let Some(c) = self.peek(found_chars).filter(|c| c.is_ascii_digit()) {
            number_str.push(c);
            found_chars += 1;
        }
        Ok((LiteralNumer(number_str.parse()?), found_chars))
    }

    fn try_func(&mut self) -> Result<(Token, usize), Box<dyn std::error::Error>> {
        let found = match (
            self.current_char(),
            self.peek(1),
            self.peek(2),
            self.peek(3),
            self.peek(4),
        ) {
            (Some('m'), Some('u'), Some('l'), _, _) => (Token::Func(Multiply), 3),
            (Some('d'), Some('o'), Some('n'), Some('\''), Some('t')) => (Token::Func(Dont), 5),
            (Some('d'), Some('o'), _, _, _) => (Token::Func(FunctionType::Do), 2),
            (_, _, _, _, _) => (Token::Unknown, 1),
        };
        Ok(found)
    }

    fn current_char(&self) -> Option<char> {
        if self.is_done() {
            return None;
        }
        self.input.chars().nth(self.current_pos)
    }
    fn advance_char(&mut self, step: usize) {
        self.current_pos += step;
    }

    fn peek(&self, step: usize) -> Option<char> {
        self.input.chars().nth(self.current_pos + step)
    }

    fn is_done(&self) -> bool {
        self.current_pos >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let day = Day3 {
            input: "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
                .to_string(),
        };
        assert_eq!(day.part1().unwrap().trim(), "161");
    }
    #[test]
    fn part2_example() {
        let day = Day3 {
            input: "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                .to_string(),
        };
        assert_eq!(day.part2().unwrap().trim(), "48");
    }
}
