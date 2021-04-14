use crate::borders::{Width, CellBorder};
use crate::cells::{Cell, CellView};

use console;
use textwrap;

pub enum Alignment {
    Default,
    Left,
    Center,
    Right
}

impl Alignment {
    fn console(&self, multiline: bool) -> console::Alignment {
        match self {
            Alignment::Default => if multiline {
                console::Alignment::Left
            } else {
                console::Alignment::Center
            },
            Alignment::Left => console::Alignment::Left,
            Alignment::Center => console::Alignment::Center,
            Alignment::Right => console::Alignment::Right
        }
    }
}

pub struct TextCell {
    text: String,
    padding: usize,
    alignment: Alignment,
    border: Width
}

impl TextCell {
    fn new(text: String, padding: usize, alignment: Alignment, border: Width) -> TextCell {
        // fuck your tabs! probably
        TextCell { text: text.replace('\t', " "), padding, alignment, border }
    }

    pub fn default(text: String, padding: usize, border: Width) -> TextCell {
        TextCell::new(text, padding, Alignment::Default, border)
    }

    pub fn centered(text: String, padding: usize, border: Width) -> TextCell {
        TextCell::new(text, padding, Alignment::Center, border)
    }

    pub fn left(text: String, padding: usize, border: Width) -> TextCell {
        TextCell::new(text, padding, Alignment::Left, border)
    }

    pub fn right(text: String, padding: usize, border: Width) -> TextCell {
        TextCell::new(text, padding, Alignment::Right, border)
    }

    fn pad(&self, line: &str) -> String {
        " ".repeat(self.padding) + line + &" ".repeat(self.padding)
    }

    fn wrap(&self, width: usize) -> Vec<String> {
        if width == 0 {
            self.text
                .lines()
                .map(|s| self.pad(s))
                .collect()
        } else {
            // TODO textwrap does not know anything about ansi codes
            let multiline = console::measure_text_width(&self.text) > width - 2 * self.padding;
            textwrap::wrap(&self.text, width - 2 * self.padding)
                .into_iter()
                .map(|s| console::pad_str(&self.pad(&s), width,
                    self.alignment.console(multiline), None).into_owned())
                .collect()
        }
    }

    fn box_align(&self, text: Vec<String>, box_height: usize, box_width: usize) -> Vec<String> {
        let text_height = text.len();
        if text_height >= box_height {
            text.into_iter()
                .take(box_height)
                .collect()
        } else {
            let diff = box_height - text_height;
            vec![" ".repeat(box_width); diff / 2].into_iter()
                .chain(text)
                .chain(vec![" ".repeat(box_width); diff - diff / 2])
                .collect()
        }
    }
}

impl Cell for TextCell {
    fn required_width(&self) -> usize { 1 + 2 * self.padding }

    fn required_width_no_wrap(&self) -> usize {
        self.text.lines()
            .map(|s| console::measure_text_width(s))
            .max()
            .unwrap_or(0)
            + 2 * self.padding
    }

    fn required_height(&self, width: usize) -> usize {
        textwrap::wrap(&self.text, width - 2 * self.padding).len()
    }

    fn draw(&self, height: usize, width: usize) -> CellView {
        let wrapped_text: Vec<String> = self.wrap(width);
        let height = if height > 0 { height } else { wrapped_text.len() };
        let width = if width > 0 { width } else { self.required_width_no_wrap() };
        let textbox = self.box_align(wrapped_text, height, width);

        CellView::new(
            height,
            width,
            textbox,
            CellBorder::atomic(height + 2, width + 2, self.border)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn textcell_created() {
        let cell = TextCell::default(String::from("Hello!"), 0, Width::None);
        assert_eq!("Hello!", cell.text);
    }

    #[test]
    fn textcell_tabs() {
        let cell = TextCell::default(String::from("Hi!\tFuck tabs!"), 0, Width::None);
        assert_eq!("Hi! Fuck tabs!", cell.text);
    }
}