use crate::{
    borders::{Border, CellBorder, Orientation},
    config::{Bound, CellConfig, Vertical},
};

use super::{Cell, CellView, Draw, DrawCell, GridSlice, GridSliceMut};

#[derive(Debug)]
pub struct Col {
    rows: Vec<Box<dyn DrawCell>>,
    config: CellConfig,
}

impl Col {
    pub fn new(rows: Vec<Box<dyn DrawCell>>, config: CellConfig) -> Self {
        let child = rows
            .iter()
            .map(|x| *x.get_config())
            .collect::<Vertical<_>>()
            .0;
        Self {
            rows,
            config: CellConfig {
                bounds: config.bounds + child.bounds,
                span_height: child.span_height,
                span_width: child.span_width,
                ..config
            },
        }
    }
}

impl Cell for Col {
    fn get_config(&self) -> &CellConfig {
        &self.config
    }
    fn get_config_mut(&mut self) -> &mut CellConfig {
        &mut self.config
    }
    fn debug_str(&self) -> String {
        format!("{:?}", self)
    }

    fn fixup_config(&mut self, row_ratio: usize, col_ratio: usize) {
        self.fixup_config_default(row_ratio, col_ratio);

        for row in self.rows.iter_mut() {
            let col_ratio = self.config.span_width / row.get_config().span_width;
            row.fixup_config(row_ratio, col_ratio);
        }
    }

    fn fixup_grid(&self, mut grid: GridSliceMut) {
        let mut start = 0;
        for row in self.rows.iter() {
            let len = row.get_config().span_height;
            row.fixup_grid(grid.slice_mut(start..(start + len), ..));
            start += len;
        }
        debug_assert_eq!(start, self.config.span_height);

        self.fixup_grid_default(grid);
    }
}

impl Draw for Col {
    fn draw(&self, grid: GridSlice) -> CellView {
        let Bound {
            pt_height,
            pt_width,
        } = grid.get_bound();

        let mut textbox: Vec<String> = Vec::with_capacity(pt_height);
        let mut total_border: Option<CellBorder> = None;

        let mut start = 0;
        for row in self.rows.iter() {
            let len = row.get_config().span_height;
            let (row_textbox, border) = row.draw(grid.slice(start..(start + len), ..)).unwrap();
            start += len;

            if !textbox.is_empty() && textbox.len() + 1 + row_textbox.len() > pt_height {
                break;
            }

            let mut separator: Option<Border> = None;
            if let Some(brd) = total_border {
                let (b, s) = brd.add_vertical(&border);
                total_border = Some(b);
                separator = Some(s);
            } else {
                total_border = Some(border);
            }

            if let Some(sep) = separator {
                let border_sep = sep.render_view(Orientation::Horizontal);
                let mut iter = border_sep.chars();
                iter.next();
                iter.next_back();

                textbox.push(iter.as_str().to_string());
            }
            textbox.extend(row_textbox);
        }

        let outer = CellBorder::atomic(pt_height + 2, pt_width + 2, self.config.border);
        CellView::new(textbox, total_border.unwrap().combine(&outer))
    }
}

#[macro_export]
macro_rules! col {
    ({$($i:ident=$e:expr),* $(,)?}, $($x:expr),* $(,)?) => {{
        let config = $crate::config::CellConfig::from(properties!($($i=$e),*));
        $crate::cells::Col::new(_array![$($x),*], config)
    }};

    ($($x:expr),*, {$($i:ident=$e:expr),* $(,)?} $(,)?) => {{
        col!({$($i=$e),*}, $($x),*)
    }};

    ($($x:expr),* $(,)?) => {{
        col!({}, $($x),*)
    }};
}
