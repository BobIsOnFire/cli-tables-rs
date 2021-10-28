use std::slice::SliceIndex;

use crate::config::{Bound, CellConfig};

#[derive(Debug)]
pub struct Grid {
    pub heights: Vec<usize>,
    pub widths: Vec<usize>,
}

#[derive(Debug)]
pub struct GridSlice<'a> {
    pub heights: &'a [usize],
    pub widths: &'a [usize],
}

#[derive(Debug)]
pub struct GridSliceMut<'a> {
    pub heights: &'a mut [usize],
    pub widths: &'a mut [usize],
}

impl Grid {
    pub fn new(config: &CellConfig) -> Grid {
        Grid {
            heights: vec![0; config.span_height],
            widths: vec![0; config.span_width],
        }
    }

    pub fn slice_mut(&mut self) -> GridSliceMut {
        GridSliceMut {
            heights: &mut self.heights,
            widths: &mut self.widths,
        }
    }

    pub fn slice(&self) -> GridSlice {
        GridSlice {
            heights: &self.heights,
            widths: &self.widths,
        }
    }
}

impl GridSliceMut<'_> {
    pub fn slice_mut<T, U>(&mut self, range_height: T, range_width: U) -> GridSliceMut
    where
        T: SliceIndex<[usize], Output = [usize]>,
        U: SliceIndex<[usize], Output = [usize]>,
    {
        GridSliceMut {
            heights: &mut self.heights[range_height],
            widths: &mut self.widths[range_width],
        }
    }

    pub fn get_bound(&self) -> Bound {
        Bound::new(
            self.heights.iter().sum::<usize>() - 1,
            self.widths.iter().sum::<usize>() - 1,
        )
    }
}

impl GridSlice<'_> {
    pub fn slice<T, U>(&self, range_height: T, range_width: U) -> GridSlice
    where
        T: SliceIndex<[usize], Output = [usize]>,
        U: SliceIndex<[usize], Output = [usize]>,
    {
        GridSlice {
            heights: &self.heights[range_height],
            widths: &self.widths[range_width],
        }
    }

    pub fn get_bound(&self) -> Bound {
        Bound::new(
            self.heights.iter().sum::<usize>() - 1,
            self.widths.iter().sum::<usize>() - 1,
        )
    }
}
