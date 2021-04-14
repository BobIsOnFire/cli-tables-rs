use crate::borders::CellBorder;
use console;

pub struct CellView {
    textbox_height: usize,
    textbox_width: usize,
    textbox: Vec<String>,
    border: CellBorder
}

impl CellView {
    pub fn new(textbox_height: usize, textbox_width: usize, textbox: Vec<String>, border: CellBorder) -> CellView {
        if textbox.len() != textbox_height {
            panic!("CellView: incorrect textbox height. Expected {}, actual {}",
                   textbox_height, textbox.len());
        }

        let mut i: usize = 0;
        for line in textbox.iter() {
            let actual_width = console::measure_text_width(&line);
            if actual_width != textbox_width {
                panic!("CellView: incorrect textbox width at line #{}. Expected {}, actual {}",
                       i, textbox_width, actual_width);
            }
            i += 1;
        }

        if !border.check_size(textbox_height + 2, textbox_width + 2) {
            panic!("CellView: incorrect border size. Expected: height {}, width {}",
                   textbox_height, textbox_width);
        }

        CellView { textbox_height, textbox_width, textbox, border }
    }

    pub fn unwrap(self) -> (Vec<String>, CellBorder) {
        (self.textbox, self.border)
    }

    pub fn complete(self) -> Vec<String> {
        self.border.render_view(&self.textbox)
    }
}

pub trait Cell {
    fn required_width(&self) -> usize;
    fn required_width_no_wrap(&self) -> usize;
    fn required_height(&self, width: usize) -> usize;
    fn draw(&self, height: usize, width: usize) -> CellView;
}
