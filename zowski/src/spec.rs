use super::parse::parse_regex;
use super::Regex;

/// Read token spec from file.
///
/// Simple file format, consisting of lines of:
/// ID: PATTERN
pub fn read_spec(filename: &str) -> Result<Vec<TokenSpec>, SpecParseError> {
    let f = std::fs::File::open(filename)?;

    let mut specs = vec![];

    let reader = std::io::BufReader::new(f);
    use std::io::BufRead;
    for (row, line) in reader.lines().enumerate() {
        let line: String = line?;
        let line = line.trim();
        if line.starts_with('#') {
            continue;
        }

        // println!("Line: {}", line);
        if line.contains(':') {
            let mut parts = line.splitn(2, ':');
            let name: String = parts.next().unwrap().trim().to_owned();
            let pattern: String = parts.next().unwrap().trim().to_owned();
            let pattern = parse_regex(pattern.as_str())
                .map_err(|e| format!("{},{}: {}", row + 1, e.index, e.message))?;
            // println!("ID = '{}' PATTERN = '{}'", id, pattern);
            specs.push(TokenSpec { name, pattern });
        }
    }

    Ok(specs)
}

#[derive(Debug)]
pub enum SpecParseError {
    Io(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for SpecParseError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<String> for SpecParseError {
    fn from(err: String) -> Self {
        Self::Other(err)
    }
}

pub struct TokenSpec {
    pub name: String,
    pub pattern: Regex,
}
