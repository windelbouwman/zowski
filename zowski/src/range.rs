//! A datatype to represent a range
//!
//! A range will have a `begin` and an `end`.
//! Given a range, we can check for overlap, if a value
//! is contained in the range.

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Range<T> {
    pub begin: T,
    pub end: T,
}

pub trait ItemsBetween {
    fn items_between(&self, other: &Self) -> usize;
    fn add_index(&self, index: usize) -> Self;
    fn sub_index(&self, index: usize) -> Self;
}

impl ItemsBetween for char {
    fn items_between(&self, other: &Self) -> usize {
        let a = *self as usize;
        let b = *other as usize;
        assert!(b >= a);
        b + 1 - a
    }

    fn add_index(&self, index: usize) -> Self {
        let a: usize = *self as usize;
        std::char::from_u32((a + index) as u32).unwrap()
    }

    fn sub_index(&self, index: usize) -> Self {
        let a: usize = *self as usize;
        std::char::from_u32((a - index) as u32).unwrap()
    }
}

impl ItemsBetween for i32 {
    fn items_between(&self, other: &Self) -> usize {
        let a = *self;
        let b = *other;
        assert!(b >= a);
        (b + 1 - a) as usize
    }

    fn add_index(&self, index: usize) -> Self {
        let a: i32 = *self;
        a + (index as i32)
    }

    fn sub_index(&self, index: usize) -> Self {
        let a: i32 = *self;
        a - (index as i32)
    }
}

impl<T> Range<T>
where
    T: PartialOrd + ItemsBetween,
{
    pub fn new(begin: T, end: T) -> Self {
        assert!(begin <= end);
        Range { begin, end }
    }

    pub fn contains(&self, c: T) -> bool {
        (self.begin <= c) && (c <= self.end)
    }

    pub fn cardinality(&self) -> usize {
        self.begin.items_between(&self.end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        (self.begin <= other.end) && (other.begin <= self.end)
    }
}

// impl<T> std::ops::Index<usize> for Range<T>
// where
//     T: ItemsBetween,
// {
//     type Output = T;

//     fn index(&self, index: usize) -> Self::Output {
//         &self.begin.add_index(index)
//     }
// }

impl<T> std::fmt::Display for Range<T>
where
    T: std::fmt::Display + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.begin == self.end {
            write!(f, "{}", self.begin)
        } else {
            write!(f, "{}-{}", self.begin, self.end)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Range;

    #[test]
    fn set_overlap1() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(1, 4);
        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn set_overlap2() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(1, 11);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn set_overlap3() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(1, 17);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn set_overlap4() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(12, 13);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn set_overlap5() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(12, 17);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn set_overlap9() {
        let r1 = Range::new(10, 14);
        let r2 = Range::new(16, 19);
        assert!(!r1.overlaps(&r2));
    }
}
