use crate::config::CellConfig;

use super::{CellView, GridSlice, GridSliceMut};

fn increase_to_size(slice: &mut [usize], size: usize) {
    let sum: usize = slice.iter().sum();
    if sum >= size { return }

    let diff = size - sum;
    let count = slice.len();
    for el in slice.iter_mut() { *el += diff / count }
    for el in slice.iter_mut().take(diff % count) { *el += 1 }

    debug_assert_eq!(slice.iter().sum::<usize>(), size)
}

pub trait Cell {
    fn get_config(&self) -> &CellConfig;
    fn get_config_mut(&mut self) -> &mut CellConfig;
    fn debug_str(&self) -> String;
    fn fixup_config(&mut self, row_ratio: usize, col_ratio: usize);
    fn fixup_grid(&self, grid: GridSliceMut);

    fn fixup_config_default(&mut self, row_ratio: usize, col_ratio: usize) {
        let config = self.get_config_mut();
        config.span_height *= row_ratio;
        config.span_width *= col_ratio;
    }
    
    fn fixup_grid_default(&self, grid: GridSliceMut) {
        let config = self.get_config();
        increase_to_size(grid.heights, config.bounds.rec.pt_height);
        increase_to_size(grid.widths, config.bounds.rec.pt_width);
    }
}

pub trait Draw {
    fn draw(&self, grid: GridSlice) -> CellView;
}

// for container elements
pub trait DrawCell: Draw + Cell {}
impl<T: Draw + Cell> DrawCell for T {}

impl std::fmt::Debug for dyn DrawCell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.debug_str())
    }
}



#[macro_export]
macro_rules! _do_array {
    ($items:expr, ) => { };

    ($items:expr, $x:expr) => {
        let item: Box<dyn $crate::cells::DrawCell> = Box::new($x);
        let items: &mut Vec<Box<dyn $crate::cells::DrawCell>> = $items;
        items.push(item);
    };

    ($items:expr, $x:expr, $($y:expr),+) => {
        _do_array![$items, $x];
        _do_array![$items, $($y),+];
    };
}

#[macro_export]
macro_rules! _array {
    ($($x:expr),*) => {{
        let mut items: Vec<Box<dyn $crate::cells::DrawCell>> = vec![];
        _do_array![&mut items, $($x),*];
        items
    }};
}
