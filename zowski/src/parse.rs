use crate::expression::Regex;

pub fn parse_regex(txt: &str) -> Result<Regex, ParseError> {
    let mut p = Parser::new(txt);
    p.parse()
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub index: usize,
}

struct Parser {
    index: usize,
    iter: Vec<char>,
}

/// Regex parser.
///
/// This thing can process a regex, one character at a time.
impl Parser {
    fn new(txt: &str) -> Self {
        Parser {
            index: 0,
            iter: txt.chars().collect(),
        }
    }

    fn parse(&mut self) -> Result<Regex, ParseError> {
        let mut expr = self.parse_one()?;
        while self.peek().is_some() {
            expr = expr + self.parse_one()?;
        }
        Ok(expr)
    }

    /// Parse element + optional repetition suffix
    fn parse_one(&mut self) -> Result<Regex, ParseError> {
        let expr = self.parse_element()?;
        self.postfix(expr)
    }

    fn parse_element(&mut self) -> Result<Regex, ParseError> {
        let c = self.get_char()?;
        match c {
            '[' => {
                // process ^ operator at start:
                let inverted: bool = self.has_taken('^');

                let mut ranges = vec![];

                loop {
                    let start = self.get_escaped_char()?;
                    let range: (char, char) = if let Some('-') = self.peek() {
                        self.get_char().expect("We have a character here");
                        let end = self.get_escaped_char()?;
                        (start, end)
                    } else {
                        (start, start)
                    };
                    ranges.push(range);

                    if let Some(']') = self.peek() {
                        break;
                    }
                }

                // let symbol_set = CharSet::from_ranges(ranges);

                self.take(']')?;

                let regex = Regex::symbol_ranges(ranges);

                if inverted {
                    Ok(regex.logical_not())
                } else {
                    Ok(regex)
                }
            }
            '(' => {
                // braced expression
                let mut elements = vec![];
                loop {
                    elements.push(self.parse_one()?);
                    if let Some(')') = self.peek() {
                        break;
                    }
                }

                assert!(!elements.is_empty());
                let (first, rest) = elements.split_first().unwrap();
                let inner: Regex = rest.iter().fold(first.clone(), |a, b| a + b.clone());
                self.take(')')?;
                Ok(inner)
            }
            '!' => {
                let inner = self.parse_element()?;
                Ok(inner.logical_not())
            }
            '.' => {
                // any character
                Ok(Regex::sigma())
            }
            '\\' => {
                // handle escape character!
                let c = Self::escape_it(self.get_char()?);
                Ok(Regex::symbol(c))
            }
            c => Ok(Regex::symbol(c)),
        }
    }

    fn postfix(&mut self, expr: Regex) -> Result<Regex, ParseError> {
        // parse suffix:
        match self.peek() {
            Some('*') => {
                // zero or more
                self.get_char().expect("We have a character here");
                Ok(expr.kleene())
            }
            Some('+') => {
                // one or more.
                self.get_char().expect("We have a character here");
                Ok(expr.one_or_more())
            }
            Some('{') => {
                unimplemented!();
            }
            _ => Ok(expr),
        }
    }

    fn escape_it(c: char) -> char {
        match c {
            'n' => '\n',
            other => other,
        }
    }

    fn get_escaped_char(&mut self) -> Result<char, ParseError> {
        let c = self.get_char()?;
        if c == '\\' {
            let c = self.get_char()?;
            Ok(Self::escape_it(c))
        } else {
            Ok(c)
        }
    }

    fn peek(&self) -> Option<&char> {
        self.iter.get(self.index)
    }

    fn has_taken(&mut self, c: char) -> bool {
        if let Some(ch) = self.peek() {
            if *ch == c {
                self.get_char().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_char(&mut self) -> Result<char, ParseError> {
        let c = self.iter.get(self.index).cloned();
        if c.is_some() {
            self.index += 1;
        }

        match c {
            Some(c) => Ok(c),
            None => Err(self.error("Expected character".to_owned())),
        }
    }

    /// Expect the given character, and proceed
    fn take(&mut self, expected_c: char) -> Result<(), ParseError> {
        let c = self.get_char()?;
        if c == expected_c {
            Ok(())
        } else {
            Err(self.error(format!("Expected {}, but got {}", expected_c, c)))
        }
    }

    fn error(&self, message: String) -> ParseError {
        ParseError {
            message,
            index: self.index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_regex, Regex};

    #[test]
    fn parse_symbol() {
        let expr = parse_regex("G").unwrap();
        let expr2 = Regex::symbol('G');
        assert_eq!(expr, expr2);
    }

    #[test]
    fn parse_postfix() {
        let expr = parse_regex("G*").unwrap();
        let expr2 = Regex::symbol('G').kleene();
        assert_eq!(expr, expr2);
    }

    #[test]
    fn parse_range() {
        let expr = parse_regex("[B-LX]").unwrap();
        let expr2 = Regex::symbol_ranges(vec![('B', 'L'), ('X', 'X')]);
        assert_eq!(expr, expr2);
    }
}
