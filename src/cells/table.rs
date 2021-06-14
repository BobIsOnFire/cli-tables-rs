use crate::borders::{Border, CellBorder, Orientation::*, Width};
use crate::cells::{Cell, CellConfig, CellView};

pub struct Row {
    cells: Vec<Box<dyn Cell>>,
    border: Width,
}

impl Row {
    pub fn new(cells: Vec<Box<dyn Cell>>, config: CellConfig) -> Row {
        Row {
            cells,
            border: config.width,
        }
    }

    fn required_height(&self, widths: &Vec<usize>) -> usize {
        self.cells
            .iter()
            .zip(widths)
            .map(|(cell, width)| cell.required_height(*width))
            .max()
            .unwrap_or(0)
    }

    fn required_width(&self) -> usize {
        self.cells
            .iter()
            .map(|cell| cell.required_width())
            .sum::<usize>()
            + self.cells.len()
            - 1
    }

    fn required_width_no_wrap(&self) -> usize {
        self.cells
            .iter()
            .map(|cell| cell.required_width_no_wrap())
            .sum::<usize>()
            + self.cells.len()
            - 1
    }

    pub fn draw(&self, mut height: usize, widths: &Vec<usize>) -> CellView {
        if height == 0 {
            height = self.required_height(widths)
        }

        let width_total = widths.iter().sum::<usize>() + self.cells.len() - 1;
        let mut textbox = vec![String::with_capacity(width_total); height];

        let mut total_border: Option<CellBorder> = None;

        for (cell, width) in self.cells.iter().zip(widths.iter()) {
            let (lines, border) = cell.draw(height, *width).unwrap();
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
                    .zip(sep.render_view(Vertical).chars().skip(1))
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

        let outer = CellBorder::atomic(height + 2, width_total + 2, self.border);
        CellView::new(
            height,
            width_total,
            textbox,
            total_border.unwrap().combine(&outer),
        )
    }
}

pub struct Table {
    rows: Vec<Box<Row>>,
    border: Width,
}

impl Table {
    pub fn new(rows: Vec<Box<Row>>, config: CellConfig) -> Table {
        Table {
            rows,
            border: config.width,
        }
    }

    fn get_widths(&self, table_width: usize) -> Vec<usize> {
        let max_cols = self
            .rows
            .iter()
            .map(|row| row.cells.len())
            .max()
            .unwrap_or(0);
        if max_cols == 0 {
            return Vec::new();
        }

        let mut max_widths = vec![0; max_cols];
        for row in self.rows.iter() {
            max_widths
                .iter_mut()
                .zip(row.cells.iter())
                .for_each(|(max, cell)| *max = std::cmp::max(*max, cell.required_width_no_wrap()));
        }

        if table_width == 0 {
            return max_widths;
        }

        let width = table_width - max_cols + 1;
        let column_sum = max_widths.iter().sum::<usize>();
        let ratio = width as f64 / column_sum as f64;
        let mut sum = 0;

        for width in max_widths.iter_mut() {
            *width = (*width as f64 * ratio) as usize;
            sum += *width;
        }

        let last = max_widths.last_mut().unwrap();
        *last = *last + width - sum;

        max_widths
    }
}

impl Cell for Table {
    fn required_width(&self) -> usize {
        self.rows
            .iter()
            .map(|x| x.required_width())
            .max()
            .unwrap_or(0)
    }

    fn required_width_no_wrap(&self) -> usize {
        self.rows
            .iter()
            .map(|x| x.required_width_no_wrap())
            .max()
            .unwrap_or(0)
    }

    fn required_height(&self, width: usize) -> usize {
        let max_widths = self.get_widths(width);
        self.rows
            .iter()
            .map(|row| row.required_height(&max_widths))
            .sum::<usize>()
            + self.rows.len()
            - 1
    }

    fn draw(&self, height: usize, width: usize) -> CellView {
        let max_widths = self.get_widths(width);

        let width = if width > 0 {
            width
        } else {
            max_widths.iter().sum::<usize>() + max_widths.len() - 1
        };

        let height = if height > 0 {
            height
        } else {
            self.rows
                .iter()
                .map(|row| row.required_height(&max_widths))
                .sum::<usize>()
                + self.rows.len()
                - 1
        };

        let mut textbox: Vec<String> = Vec::with_capacity(height);
        let mut total_border: Option<CellBorder> = None;

        for row in self.rows.iter() {
            let (row_textbox, border) = row.draw(0, &max_widths).unwrap();
            if textbox.len() > 0 && textbox.len() + 1 + row_textbox.len() > height {
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
                let border_sep = sep.render_view(Horizontal);
                let mut iter = border_sep.chars();
                iter.next();
                iter.next_back();

                textbox.push(iter.as_str().to_string());
            }
            textbox.extend(row_textbox);
        }

        let outer = CellBorder::atomic(height + 2, width + 2, self.border);
        CellView::new(
            height,
            width,
            textbox,
            total_border.unwrap().combine(&outer),
        )
    }
}

#[macro_export]
macro_rules! _do_array {
    ($items:expr, <$item:ty>) => { };

    ($items:expr, <$item:ty> $x:expr) => {
        let item: Box<$item> = Box::new($x);
        let items: &mut Vec<Box<$item>> = $items;
        items.push(item);
    };

    ($items:expr, <$item:ty> $x:expr, $($y:expr),+) => {
        _do_array![$items, <$item> $x];
        _do_array![$items, <$item> $($y),+];
    };
}

#[macro_export]
macro_rules! _array {
    (<$cont:ty, $item:ty> $config:expr, $($x:expr),*) => {{
        let mut items: Vec<Box<$item>> = vec![];
        _do_array![&mut items, <$item> $($x),*];
        <$cont>::new(items, $config)
    }};
}

#[macro_export]
macro_rules! row {
    ({$($i:ident=$e:expr),* $(,)?}, $($x:expr),* $(,)?) => {{
        #[allow(unused_mut)] // Compiler does not understand that it actually has to be mutable
        let mut config = $crate::cells::CellConfig::default();

        cellconfig!(config, $($i=$e),*);
        _array![<$crate::cells::Row, dyn $crate::cells::Cell> config, $($x),*]
    }};

    ($($x:expr),*, {$($i:ident=$e:expr),* $(,)?} $(,)?) => {{
        row!({$($i=$e),*}, $($x),*)
    }};

    ($($x:expr),* $(,)?) => {{
        row!({}, $($x),*)
    }};
}

#[macro_export]
macro_rules! table {
    ({$($i:ident=$e:expr),* $(,)?}, $($x:expr),* $(,)?) => {{
        #[allow(unused_mut)] // Compiler does not understand that it actually has to be mutable
        let mut config = $crate::cells::CellConfig::default();

        cellconfig!(config, $($i=$e),*);
        _array![<$crate::cells::Table, $crate::cells::Row> config, $($x),*]
    }};

    ($($x:expr),*, {$($i:ident=$e:expr),* $(,)?} $(,)?) => {{
        table!({$($i=$e),*}, $($x),*)
    }};

    ($($x:expr),* $(,)?) => {{
        table!({}, $($x),*)
    }};
}
