use std::ops::{Add, AddAssign};
use std::iter::FromIterator;

use crate::config::utils::*;

#[derive(Clone, Copy, Debug)]
pub struct Bound {
    pub pt_height: usize,
    pub pt_width: usize
}

impl Bound {
    pub fn new(pt_height: usize, pt_width: usize) -> Self {
        Self { pt_height, pt_width }
    }
}

impl FromIterator<Bound> for Vertical<Bound> {
    fn from_iter<I: IntoIterator<Item = Bound>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(Bound {
            pt_height: elems.iter().map(|x| x.pt_height).sum(),
            pt_width: elems.iter().map(|x| x.pt_width).max().unwrap_or(0),
        })
    }
}

impl FromIterator<Bound> for Horizontal<Bound> {
    fn from_iter<I: IntoIterator<Item = Bound>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(Bound {
            pt_height: elems.iter().map(|x| x.pt_height).max().unwrap_or(0),
            pt_width: elems.iter().map(|x| x.pt_width).sum()
        })
    }
}

impl Add for Bound {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            pt_height: max(self.pt_height, other.pt_height),
            pt_width: max(self.pt_width, other.pt_width),
        }
    }
}

impl AddAssign for Bound {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
