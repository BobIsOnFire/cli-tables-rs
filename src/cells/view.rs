use crate::borders::CellBorder;

pub struct CellView {
    textbox: Vec<String>,
    border: CellBorder,
}

impl CellView {
    pub fn new(
        textbox: Vec<String>,
        border: CellBorder,
    ) -> Self {
        let iter = textbox.iter().map(|x| console::measure_text_width(&x));

        debug_assert_eq!(iter.clone().min().unwrap_or(0), iter.clone().max().unwrap_or(0));
        debug_assert!(border.check_size(textbox.len() + 2, iter.clone().max().unwrap_or(0) + 2));

        Self {
            textbox,
            border,
        }
    }

    pub fn unwrap(self) -> (Vec<String>, CellBorder) {
        (self.textbox, self.border)
    }

    pub fn complete(self) -> Vec<String> {
        self.border.render_view(&self.textbox)
    }
}
