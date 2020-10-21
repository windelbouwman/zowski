//! Implementation of integer sets of by using ranges.
//!
//! Heavily copied from:
//! https://github.com/MichaelPaddon/epsilon/blob/master/epsilon/util.py

use super::range::{ItemsBetween, Range};

/// A set of integers represented internally
/// as a sequence of ranges.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeSet<T> {
    ranges: Vec<Range<T>>,
}

fn glue_overlapping<T>(ranges: Vec<Range<T>>) -> Vec<Range<T>>
where
    T: Copy + ItemsBetween + Ord,
{
    if ranges.is_empty() {
        ranges
    } else {
        let mut glued_ranges: Vec<Range<T>> = vec![];
        let (first, rest) = ranges.split_first().unwrap();
        let mut range: Range<T> = first.clone();
        for r2 in rest {
            let end_plus_one = range.end.add_index(1);
            if r2.begin > end_plus_one {
                glued_ranges.push(range);
                range = r2.clone();
            } else {
                range.end = range.end.max(r2.end);
            }
        }
        glued_ranges.push(range);
        glued_ranges
    }
}

impl<T> RangeSet<T>
where
    T: Copy + Ord + ItemsBetween + std::fmt::Display + std::fmt::Debug,
{
    /// Create a new set containing a single element.
    pub fn new(c: T) -> Self {
        Self::from_ranges(vec![Range::new(c, c)])
    }

    /// Create a new set containing a single range.
    ///
    /// TODO: think of sensible naming!
    pub fn new2(from: T, to: T) -> Self {
        Self::from_ranges(vec![Range::new(from, to)])
    }

    /// Create a set from the given ranges.
    ///
    /// Ranges will be sorted and glued together if possible.
    pub fn from_ranges(mut ranges: Vec<Range<T>>) -> Self {
        ranges.sort();
        let ranges = glue_overlapping(ranges);
        RangeSet { ranges }
    }

    /// Create the empty set
    pub fn empty() -> Self {
        Self::from_ranges(vec![])
    }

    /// Check if this set is empty
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn cardinality(&self) -> usize {
        let mut s = 0;
        for r in &self.ranges {
            s += r.cardinality();
        }
        s
    }

    /// Retrieve the first character in the set
    ///
    /// Panics when set is empty
    pub fn first(&self) -> T {
        self.ranges.first().unwrap().begin
    }

    pub fn contains(&self, c: T) -> bool {
        for r in &self.ranges {
            if r.contains(c) {
                return true;
            }
        }
        false
    }

    /// Return union of this set with another set.
    pub fn union(&self, other: Self) -> Self {
        let mut new_ranges = self.ranges.clone();
        new_ranges.extend(other.ranges);
        Self::from_ranges(new_ranges)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut self_iter = self.ranges.iter();
        let mut other_iter = other.ranges.iter();
        let mut self_opt: Option<&Range<T>> = self_iter.next();
        let mut other_opt: Option<&Range<T>> = other_iter.next();

        // println!("Well. {}");

        let mut resulting_ranges: Vec<Range<T>> = vec![];
        while let (Some(self_range), Some(other_range)) = (self_opt, other_opt) {
            let begin: T = self_range.begin.max(other_range.begin);
            let end: T = self_range.end.min(other_range.end);
            if begin <= end {
                resulting_ranges.push(Range::new(begin, end));
            }

            if self_range.end <= end {
                self_opt = self_iter.next();
            }

            if other_range.end <= end {
                other_opt = other_iter.next();
            }
        }

        RangeSet::from_ranges(resulting_ranges)
    }

    /// Determine the symmetric difference between two sets.
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        self.difference(other).union(other.difference(self))
    }

    /// Calculate the set difference between this set and
    /// the other set.
    pub fn difference(&self, other: &Self) -> Self {
        let mut self_iter = self.ranges.iter();
        let mut other_iter = other.ranges.iter();
        let mut self_opt: Option<Range<T>> = self_iter.next().cloned();
        let mut other_opt: Option<&Range<T>> = other_iter.next();

        let mut resulting_ranges: Vec<Range<T>> = vec![];
        while let (Some(self_range), Some(other_range)) = (&self_opt, other_opt) {
            if other_range.end < self_range.begin {
                // other range before this range
                other_opt = other_iter.next();
            } else if self_range.end < other_range.begin {
                // this range before other range
                resulting_ranges.push(self_range.clone());
                self_opt = self_iter.next().cloned();
            } else {
                // Okay, we have overlap!
                assert!(self_range.overlaps(other_range));

                // First check if r2 is before
                if self_range.begin < other_range.begin {
                    resulting_ranges
                        .push(Range::new(self_range.begin, other_range.begin.sub_index(1)));
                }

                if self_range.end > other_range.end {
                    self_opt = Some(Range::new(other_range.end.add_index(1), self_range.end));
                    other_opt = other_iter.next();
                } else {
                    self_opt = self_iter.next().cloned();
                }
            }
        }

        if let Some(self_range) = self_opt {
            resulting_ranges.push(self_range);
        }

        // Append all remaining ranges:
        resulting_ranges.extend(self_iter.map(|r| r.clone()));

        RangeSet::from_ranges(resulting_ranges)
    }
}

pub struct RangeSetIterator<'s, T> {
    s: &'s RangeSet<T>,
    index: usize,
    subindex: usize,
}

impl<'s, T> IntoIterator for &'s RangeSet<T>
where
    T: PartialOrd + ItemsBetween,
{
    type Item = T;
    type IntoIter = RangeSetIterator<'s, T>;

    fn into_iter(self) -> Self::IntoIter {
        RangeSetIterator {
            s: self,
            index: 0,
            subindex: 0,
        }
    }
}

impl<'s, T> Iterator for RangeSetIterator<'s, T>
where
    T: PartialOrd + ItemsBetween,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index < self.s.ranges.len() {
                let range = &self.s.ranges[self.index];
                if self.subindex < range.cardinality() {
                    let c = range.begin.add_index(self.subindex);
                    self.subindex += 1;
                    break Some(c);
                } else {
                    self.index += 1;
                    self.subindex = 0;
                }
            } else {
                break None;
            }
        }
    }
}

impl<T> std::fmt::Display for RangeSet<T>
where
    T: std::fmt::Display + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for range in &self.ranges {
            write!(f, "{}", range)?;
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RangeSet;

    #[test]
    fn set_with_single_element() {
        let s = RangeSet::new('A');
        assert!(!s.is_empty());
        assert_eq!(1, s.cardinality());
        assert!(s.contains('A'));
        assert!(!s.contains('B'));
    }

    #[test]
    fn set_with_one_range() {
        let s = RangeSet::new2('A', 'G');
        assert!(!s.is_empty());
        assert_eq!(7, s.cardinality());
        assert!(s.contains('A'));
        assert!(s.contains('B'));
        assert!(!s.contains('Z'));
        assert!(!s.contains('7'));
    }

    #[test]
    fn set_union() {
        let s1 = RangeSet::new2('A', 'G');
        let s2 = RangeSet::new2('X', 'Z');
        let s3 = s1.union(s2);
        assert!(!s3.is_empty());
        assert_eq!(10, s3.cardinality());
        assert!(s3.contains('A'));
        assert!(s3.contains('B'));
        assert!(s3.contains('Z'));
        assert!(!s3.contains('7'));
        use std::iter::FromIterator;
        assert_eq!(
            Vec::from_iter(&s3),
            vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'X', 'Y', 'Z']
        );
    }

    #[test]
    fn set_union_glueing() {
        let s1 = RangeSet::new2('A', 'D');
        let s2 = RangeSet::new2('E', 'K');
        let s3 = s1.union(s2);
        assert!(!s3.is_empty());
        assert_eq!(s3, RangeSet::new2('A', 'K'));
    }

    #[test]
    fn set_difference() {
        let s1 = RangeSet::new2('A', 'G');
        let s2 = RangeSet::new2('D', 'Z');
        let s3 = s1.difference(&s2);
        assert!(!s3.is_empty());
        assert_eq!(3, s3.cardinality());
        assert!(s3.contains('A'));
        assert!(s3.contains('B'));
        assert!(!s3.contains('Z'));
        assert!(!s3.contains('7'));
        assert_eq!(s3, RangeSet::new2('A', 'C'));
    }

    #[test]
    fn set_difference2() {
        let s1 = RangeSet::new2('A', 'Z');
        let s2 = RangeSet::new2('0', '9');
        let s3 = s1.difference(&s2);
        println!("S3= {}", s3);
        assert!(!s3.is_empty());
        assert_eq!(s3, RangeSet::new2('A', 'Z'));
    }

    #[test]
    fn set_intersection() {
        let s1 = RangeSet::new2('A', 'G');
        let s2 = RangeSet::new2('D', 'Z');
        let s3 = s1.intersection(&s2);
        assert!(!s3.is_empty());
        assert_eq!(4, s3.cardinality());
        assert!(!s3.contains('A'));
        assert!(!s3.contains('B'));
        assert!(s3.contains('D'));
        assert!(s3.contains('F'));
        assert!(!s3.contains('Z'));
        assert!(!s3.contains('7'));
        assert_eq!(s3, RangeSet::new2('D', 'G'));
    }

    #[test]
    fn set_iteration() {
        let s = RangeSet::new2('A', 'G');
        let mut res = vec![];
        for c in &s {
            res.push(c);
        }
        assert_eq!(res, vec!['A', 'B', 'C', 'D', 'E', 'F', 'G']);
    }
}
