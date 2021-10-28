use crate::{borders::CellBorder, config::{Alignment, Bound, CellBounds, CellConfig}};

use super::{Cell, CellView, Draw, GridSlice, GridSliceMut};

#[derive(Debug)]
pub struct TextCell {
    text: String,
    config: CellConfig
}

impl TextCell {
    pub fn new(text: String, config: CellConfig) -> Self {
        // fuck your tabs! probably
        let text = text.replace('\t', " ");
        Self {
            text: text.replace('\t', " "),
            config: CellConfig {
                bounds: config.bounds + CellBounds::from_text(&text, config.padding),
                ..config
            }
        }
    }
}

impl Cell for TextCell {
    fn get_config(&self) -> &CellConfig { &self.config }
    fn get_config_mut(&mut self) -> &mut CellConfig { &mut self.config }
    fn debug_str(&self) -> String { format!("{:?}", self) }
    fn fixup_config(&mut self, row_ratio: usize, col_ratio: usize) { self.fixup_config_default(row_ratio, col_ratio) }
    fn fixup_grid(&self, grid: GridSliceMut) { self.fixup_grid_default(grid) }
}

fn pad(text: &str, padding: usize) -> String {
    " ".repeat(padding) + text + &" ".repeat(padding)
}

fn wrap(text: &String, width: usize, padding: usize, alignment: Alignment) -> Vec<String> {
    if width == 0 {
        text.lines().map(|s| pad(s, padding)).collect()
    } else {
        // TODO there are at least two fatal flaws in this code!
        let multiline = console::measure_text_width(&text) > width - 2 * padding;
        textwrap::wrap(&text, width - 2 * padding)
            .into_iter()
            .map(|s| {
                console::pad_str(
                    &pad(&s, padding),
                    width,
                    alignment.console(multiline),
                    None,
                )
                .into_owned()
            })
            .collect()
    }
}

fn box_align(text: Vec<String>, box_height: usize, box_width: usize) -> Vec<String> {
    let text_height = text.len();
    if text_height >= box_height {
        text.into_iter().take(box_height).collect()
    } else {
        let diff = box_height - text_height;
        vec![" ".repeat(box_width); diff / 2]
            .into_iter()
            .chain(text)
            .chain(vec![" ".repeat(box_width); diff - diff / 2])
            .collect()
    }
}


impl Draw for TextCell {
    fn draw(&self, grid: GridSlice) -> CellView {
        let Bound { pt_height, pt_width } = grid.get_bound();
        let wrapped_text = wrap(&self.text, pt_width, self.config.padding, self.config.alignment);
        let textbox = box_align(wrapped_text, pt_height, pt_width);

        CellView::new(
            textbox,
            CellBorder::atomic(pt_height + 2, pt_width + 2, self.config.border),
        )
    }
}

#[macro_export]
macro_rules! textcell {
    ({$($i:ident=$e:expr),*}, $x:expr) => {{
        let content: String = format!("{}", $x);
        let config = $crate::config::CellConfig::from(properties!(border = Light, $($i=$e),*));

        $crate::cells::TextCell::new(content, config)
    }};

    ($x:expr, {$($i:ident=$e:expr),*}) => {{
        textcell!({$($i=$e),*}, $x)
    }};

    ($x:expr) => {{
        textcell!({}, $x)
    }};
}
