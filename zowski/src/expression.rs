use crate::range::Range;
use crate::rangeset::RangeSet;

pub type CharSet = RangeSet<char>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Regex {
    /// The empty string
    Epsilon,

    /// A set of symbols.
    ///
    /// Roughly three cases:
    /// - 0 symbols, the null set
    /// - 1 symbol, a single character
    /// - more symbols, a symbol set
    SymbolSet(CharSet),

    /// The Kleene closure operator.
    Kleene(Box<Regex>),

    /// Logical or operation
    ///
    /// Match either left of right arms
    Alternation {
        left: Box<Regex>,
        right: Box<Regex>,
    },

    /// Concatenation
    ///
    /// Match left followed by right
    Concatenation {
        left: Box<Regex>,
        right: Box<Regex>,
    },

    /// Logical AND operation
    ///
    /// Match left and right arms
    LogicalAnd {
        left: Box<Regex>,
        right: Box<Regex>,
    },

    LogicalNot(Box<Regex>),
}

impl Regex {
    /// Apply the Kleene closure operator to this regex.
    pub fn kleene(self) -> Self {
        if self.is_epsilon() {
            self
        } else {
            Regex::Kleene(Box::new(self))
        }
    }

    /// Apply the + operator to this regex
    pub fn one_or_more(self) -> Self {
        self.clone() + self.kleene()
    }

    /// Invert the regex.
    pub fn logical_not(self) -> Self {
        match self {
            Regex::LogicalNot(r) => *r,
            Regex::SymbolSet(s) => Regex::SymbolSet(sigma().difference(&s)),
            other => Regex::LogicalNot(Box::new(other)),
        }
    }

    /// Create the empty string
    pub fn epsilon() -> Self {
        Regex::Epsilon
    }

    /// Create an expression matching the given char.
    pub fn symbol(c: char) -> Self {
        Regex::SymbolSet(CharSet::new(c))
    }

    /// Create expression which matches a range of characters.
    pub fn symbol_range(begin: char, end: char) -> Self {
        Regex::SymbolSet(CharSet::new2(begin, end))
    }

    pub fn symbol_ranges(ranges: Vec<(char, char)>) -> Self {
        let ranges: Vec<Range<char>> = ranges.into_iter().map(|r| Range::new(r.0, r.1)).collect();
        Regex::SymbolSet(CharSet::from_ranges(ranges))
    }

    /// Return the empty set
    pub fn null() -> Self {
        Regex::SymbolSet(CharSet::empty())
    }

    /// Match any char in the alfabet
    pub fn sigma() -> Self {
        Regex::SymbolSet(sigma())
    }

    /// Test if this regex is the empty string (epsilon)
    pub fn is_epsilon(&self) -> bool {
        matches!(self, Regex::Epsilon)
    }

    /// Test if this regex is the null set.
    pub fn is_null(&self) -> bool {
        match self {
            Regex::SymbolSet(s) => s.is_empty(),
            _ => false,
        }
    }

    /// Determine if this regex is nullable.
    ///
    /// This means that the regex can be reduced
    /// to the empty string.
    pub fn is_nullable(&self) -> bool {
        match self {
            Regex::Epsilon => true,
            Regex::Alternation { left, right } => left.is_nullable() || right.is_nullable(),
            Regex::LogicalAnd { left, right } => left.is_nullable() && right.is_nullable(),
            Regex::Concatenation { left, right } => left.is_nullable() && right.is_nullable(),
            Regex::Kleene(_) => true,
            Regex::SymbolSet(_) => false,
            Regex::LogicalNot(r) => !r.is_nullable(),
        }
    }

    /// Construct the derivative of this regex with respect to
    /// some other character.
    pub fn derivative(&self, c: char) -> Regex {
        match self {
            Regex::SymbolSet(s) => {
                if s.contains(c) {
                    Regex::epsilon()
                } else {
                    Regex::null()
                }
            }
            Regex::Epsilon => Regex::null(),
            Regex::Kleene(r) => r.derivative(c) + r.clone().kleene(),
            Regex::Alternation { left, right } => left.derivative(c) | right.derivative(c),
            Regex::LogicalAnd { left, right } => left.derivative(c) & right.derivative(c),
            Regex::Concatenation { left, right } => {
                if left.is_nullable() {
                    (left.derivative(c) + *right.clone()) | right.derivative(c)
                } else {
                    left.derivative(c) + *right.clone()
                }
            }
            Regex::LogicalNot(r) => r.derivative(c).logical_not(),
        }
    }

    /// Get a list of sets of characters that are of
    /// interest.
    pub fn character_classes(&self) -> Vec<CharSet> {
        match self {
            Regex::SymbolSet(s) => {
                if s.is_empty() {
                    vec![sigma()]
                } else {
                    vec![s.clone(), sigma().difference(s)]
                }
            }
            Regex::Kleene(r) => r.character_classes(),
            Regex::Epsilon => vec![sigma()],
            Regex::LogicalAnd { left, right } => {
                product_intersections(left.character_classes(), right.character_classes())
            }
            Regex::Alternation { left, right } => {
                product_intersections(left.character_classes(), right.character_classes())
            }
            Regex::Concatenation { left, right } => {
                if left.is_nullable() {
                    product_intersections(left.character_classes(), right.character_classes())
                } else {
                    left.character_classes()
                }
            }
            Regex::LogicalNot(r) => r.character_classes(),
        }
    }
}

/// Return whole alphabeth
fn sigma() -> CharSet {
    // TODO: expand to more code points!
    // space = ' ' = 32
    // tilde = '~' = 126
    CharSet::new2(' ', '~') | CharSet::new('\n')
}

/// Yeah...
///
/// This takes two sets of character classes, and returns the cartesian
/// intersection of them.
pub fn product_intersections(class1: Vec<CharSet>, class2: Vec<CharSet>) -> Vec<CharSet> {
    let mut intersections = vec![];
    for a in &class1 {
        for b in &class2 {
            let res = a.intersection(b);
            if !res.is_empty() {
                intersections.push(res);
            }
        }
    }

    assert_eq!(unify(intersections.clone()), sigma());
    // println!("Intersecion of {:?} and {:?}: {:?}", class1, class2, intersections);
    intersections
}

/// Apply union operation on all char sets
fn unify(s: Vec<CharSet>) -> CharSet {
    let (first, rest) = s.split_first().unwrap();
    rest.iter().fold(first.clone(), |a, b| b.union(&a))
}

impl std::ops::BitOr for Regex {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        alternation(self, rhs)
    }
}

impl std::ops::Add for Regex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        concatenate(self, rhs)
    }
}

impl std::ops::BitAnd for Regex {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        logical_and(self, rhs)
    }
}

impl From<&str> for Regex {
    fn from(re: &str) -> Self {
        use crate::parse::parse_regex;
        parse_regex(re).unwrap()
    }
}

/// Alternation / logical or operation
fn alternation(left: Regex, right: Regex) -> Regex {
    if left.is_null() {
        right
    } else if right.is_null() {
        left
    } else if let (Regex::SymbolSet(a), Regex::SymbolSet(b)) = (&left, &right) {
        Regex::SymbolSet(a.union(b))
    } else {
        Regex::Alternation {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

fn logical_and(left: Regex, right: Regex) -> Regex {
    if left.is_null() {
        left
    } else if right.is_null() {
        right
    } else {
        Regex::LogicalAnd {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

fn concatenate(left: Regex, right: Regex) -> Regex {
    if left.is_null() {
        left
    } else if right.is_null() {
        right
    } else if left.is_epsilon() {
        right
    } else if right.is_epsilon() {
        left
    } else {
        Regex::Concatenation {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

impl std::fmt::Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Regex::Epsilon => write!(f, "eps"),
            Regex::SymbolSet(s) => write!(f, "[{}]", s),
            Regex::Alternation { left, right } => write!(f, "({}|{})", left, right),
            Regex::Concatenation { left, right } => write!(f, "({}.{})", left, right),
            Regex::LogicalAnd { left, right } => write!(f, "({}&{})", left, right),
            Regex::Kleene(r) => write!(f, "{}*", r),
            Regex::LogicalNot(r) => write!(f, "!({})", r),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Regex;

    #[test]
    fn blabla1() {
        let expr = Regex::symbol('A');
        assert!(!expr.is_nullable());
        assert!(expr.derivative('A').is_epsilon());
        assert!(expr.derivative('B').is_null());
    }

    #[test]
    fn blabla2() {
        let expr = Regex::symbol('A') + Regex::symbol('B');
        assert!(!expr.is_nullable());
        assert!(expr.derivative('A') == Regex::symbol('B'));
        assert!(expr.derivative('B').is_null());
    }
}
