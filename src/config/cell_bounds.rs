use std::iter::FromIterator;
use std::ops::{Add, AddAssign};

use crate::config::utils::*;
use crate::config::Bound;

#[derive(Clone, Copy, Debug)]
pub struct CellBounds {
    pub min: Bound,
    pub rec: Bound,
}

impl CellBounds {
    pub fn new(min: Bound, rec: Bound) -> Self {
        Self { min, rec }
    }

    pub fn make_clean() -> Self {
        Self::new(Bound::new(0, 0), Bound::new(0, 0))
    }

    pub fn from_text(text: &str, padding: usize) -> Self {
        Self::new(
            Bound::new(2, 2 + 2 * padding),
            Bound::new(2, text.len() + 1 + 2 * padding),
        )
    }
}

impl FromIterator<CellBounds> for Vertical<CellBounds> {
    fn from_iter<I: IntoIterator<Item = CellBounds>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(CellBounds {
            min: elems.iter().map(|x| x.min).collect::<Vertical<_>>().0,
            rec: elems.iter().map(|x| x.rec).collect::<Vertical<_>>().0,
        })
    }
}

impl FromIterator<CellBounds> for Horizontal<CellBounds> {
    fn from_iter<I: IntoIterator<Item = CellBounds>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(CellBounds {
            min: elems.iter().map(|x| x.min).collect::<Horizontal<_>>().0,
            rec: elems.iter().map(|x| x.rec).collect::<Horizontal<_>>().0,
        })
    }
}

impl Add for CellBounds {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            min: self.min + other.min,
            rec: self.rec + other.rec,
        }
    }
}

impl AddAssign for CellBounds {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
