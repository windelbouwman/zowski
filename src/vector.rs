use crate::expression::{product_intersections, CharSet, Regex};

/// A vector of regular expressions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionVector {
    expressions: Vec<Regex>,
}

impl ExpressionVector {
    pub fn new(expressions: Vec<Regex>) -> Self {
        ExpressionVector { expressions }
    }

    /// Contrapt the derivative of this expression vector with
    /// respect to the given character.
    pub fn derivative(&self, c: char) -> ExpressionVector {
        let e2 = self.expressions.iter().map(|e| e.derivative(c)).collect();
        ExpressionVector::new(e2)
    }

    pub fn is_nullable(&self) -> bool {
        self.expressions.iter().any(|e| e.is_nullable())
    }

    /// Retrieve the different character classes that might
    /// introduce different behavior.
    pub fn character_classes(&self) -> Vec<CharSet> {
        let x: Vec<Vec<CharSet>> = self
            .expressions
            .iter()
            .map(|e| e.character_classes())
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
        for expression in &self.expressions {
            writeln!(f, " -> {}", expression)?;
        }
        Ok(())
    }
}
