use crate::{borders::{Border, CellBorder, Orientation}, config::{Bound, CellConfig, Horizontal}};

use super::{Cell, CellView, Draw, DrawCell, GridSlice, GridSliceMut};

#[derive(Debug)]
pub struct Row {
    cols: Vec<Box<dyn DrawCell>>,
    config: CellConfig
}

impl Row {
    pub fn new(cols: Vec<Box<dyn DrawCell>>, config: CellConfig) -> Self {
        let child = cols.iter().map(|x| x.get_config().clone()).collect::<Horizontal<_>>().0;
        Self {
            cols,
            config: CellConfig {
                bounds: config.bounds + child.bounds,
                span_height: child.span_height,
                span_width: child.span_width,
                ..config
            }
        }
    }
}

impl Cell for Row {
    fn get_config(&self) -> &CellConfig { &self.config }
    fn get_config_mut(&mut self) -> &mut CellConfig { &mut self.config }
    fn debug_str(&self) -> String { format!("{:?}", self) }

    fn fixup_config(&mut self, row_ratio: usize, col_ratio: usize) {
        self.fixup_config_default(row_ratio, col_ratio);

        for col in self.cols.iter_mut() {
            let row_ratio = self.config.span_height / col.get_config().span_height;
            col.fixup_config(row_ratio, col_ratio);
        }
    }

    fn fixup_grid(&self, mut grid: GridSliceMut) {
        let mut start = 0;
        for col in self.cols.iter() {
            let len = col.get_config().span_width;
            col.fixup_grid(grid.slice_mut(.., start..(start + len)));
            start += len;
        }
        debug_assert_eq!(start, self.config.span_width);

        self.fixup_grid_default(grid);
    }
}

impl Draw for Row {
    fn draw(&self, grid: GridSlice) -> CellView {
        let Bound { pt_height, pt_width } = grid.get_bound();

        let mut textbox = vec![String::with_capacity(pt_width); pt_height];
        let mut total_border: Option<CellBorder> = None;

        let mut start = 0;
        // TODO: all this fuckery has to be done in CellView
        for col in self.cols.iter() {
            let len = col.get_config().span_width;
            let (lines, border) = col.draw(grid.slice(.., start..(start + len))).unwrap();
            start += len;

            let mut separator: Option<Border> = None;

            if let Some(brd) = total_border {
                let (b, s) = brd.add_horizontal(&border);
                total_border = Some(b);
                separator = Some(s);
            } else {
                total_border = Some(border);
            }

            if let Some(sep) = separator {
                textbox
                    .iter_mut()
                    .zip(lines)
                    .zip(sep.render_view(Orientation::Vertical).chars().skip(1))
                    .for_each(|((buffer, cell), border)| {
                        buffer.push(border);
                        buffer.push_str(&cell);
                    })
            } else {
                textbox
                    .iter_mut()
                    .zip(lines)
                    .for_each(|(buffer, cell)| buffer.push_str(&cell));
            }
        }

        let outer = CellBorder::atomic(pt_height + 2, pt_width + 2, self.config.border);

        CellView::new(
            textbox,
            total_border.unwrap().combine(&outer),
        )
    }
}

#[macro_export]
macro_rules! row {
    ({$($i:ident=$e:expr),* $(,)?}, $($x:expr),* $(,)?) => {{
        let config = $crate::config::CellConfig::from(properties!($($i=$e),*));
        $crate::cells::Row::new(_array![$($x),*], config)
    }};

    ($($x:expr),*, {$($i:ident=$e:expr),* $(,)?} $(,)?) => {{
        row!({$($i=$e),*}, $($x),*)
    }};

    ($($x:expr),* $(,)?) => {{
        row!({}, $($x),*)
    }};
}
