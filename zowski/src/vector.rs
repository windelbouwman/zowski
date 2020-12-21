use crate::expression::{product_intersections, CharSet, Regex};

/// A vector of regular expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionVector {
    expressions: Vec<(String, Regex)>,
}

impl ExpressionVector {
    pub fn new(expressions: Vec<(String, Regex)>) -> Self {
        ExpressionVector { expressions }
    }

    /// Retrieve names of the expressions
    pub fn names(&self) -> Vec<String> {
        self.expressions.iter().map(|e| e.0.clone()).collect()
    }

    /// Contrapt the derivative of this expression vector with
    /// respect to the given character.
    pub fn derivative(&self, c: char) -> ExpressionVector {
        let e2 = self
            .expressions
            .iter()
            .map(|(n, e)| (n.to_owned(), e.derivative(c)))
            .collect();
        ExpressionVector::new(e2)
    }

    /// Retrieve which patterns are matched by this state.
    pub fn is_nullable(&self) -> Vec<String> {
        self.expressions
            .iter()
            .filter(|e| e.1.is_nullable())
            .map(|e| e.0.to_owned())
            .collect()
    }

    /// Test if all patterns are null
    pub fn is_null(&self) -> bool {
        self.expressions.iter().all(|e| e.1.is_null())
    }

    /// Retrieve the different character classes that might
    /// introduce different behavior.
    pub fn character_classes(&self) -> Vec<CharSet> {
        let x: Vec<Vec<CharSet>> = self
            .expressions
            .iter()
            .map(|e| e.1.character_classes())
            .collect();
        // .fold();
        if x.is_empty() {
            vec![]
        } else {
            // println!("X={}", x.len());
            let (first, rest) = x.split_first().unwrap();
            rest.iter()
                .fold(first.clone(), |a, b| product_intersections(a, b.clone()))
        }
    }
}

impl std::fmt::Display for ExpressionVector {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Vector:")?;
        for (name, expression) in &self.expressions {
            writeln!(f, " {} -> {}", name, expression)?;
        }
        Ok(())
    }
}
