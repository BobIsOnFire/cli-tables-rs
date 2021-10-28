use super::{DrawCell, Grid};

#[derive(Debug)]
pub struct Table<T: DrawCell> {
    cell: T,
    grid: Grid,
}

impl<T: DrawCell> From<T> for Table<T> {
    fn from(mut cell: T) -> Self {
        cell.fixup_config(1, 1);
        let mut grid = Grid::new(cell.get_config());
        cell.fixup_grid(grid.slice_mut());

        Self { cell, grid }
    }
}

impl<T: DrawCell> Table<T> {
    pub fn print(&self) {
        for line in self.cell.draw(self.grid.slice()).complete() {
            println!("{}", line)
        }
    }
}
