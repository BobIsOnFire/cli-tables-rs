use crate::borders::Width;

#[derive(Clone, Copy)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone)]
struct BorderFragment {
    length: usize,
    width: Width,
    left_start: Width,
    right_start: Width,
}

impl BorderFragment {
    fn new(length: usize, width: Width, left_start: Width, right_start: Width) -> BorderFragment {
        BorderFragment {
            length,
            width,
            left_start,
            right_start,
        }
    }

    fn render_view(&self, start: &Width, orientation: Orientation) -> String {
        let mut view = String::with_capacity(self.length);
        match orientation {
            Orientation::Vertical => {
                view.push(Width::draw_char(
                    start,
                    &self.left_start,
                    &self.width,
                    &self.right_start,
                ));
                for _ in 1..self.length {
                    view.push(Width::draw_char(
                        &self.width,
                        &Width::None,
                        &self.width,
                        &Width::None,
                    ));
                }
            }
            Orientation::Horizontal => {
                view.push(Width::draw_char(
                    &self.left_start,
                    start,
                    &self.right_start,
                    &self.width,
                ));
                for _ in 1..self.length {
                    view.push(Width::draw_char(
                        &Width::None,
                        &self.width,
                        &Width::None,
                        &self.width,
                    ));
                }
            }
        }

        view
    }
}

#[derive(Clone)]
pub struct Border {
    fragments: Vec<BorderFragment>,
    length: usize,
}

impl Border {
    fn new(fragments: Vec<BorderFragment>, length: usize) -> Border {
        Border { fragments, length }
    }

    pub fn default_left(length: usize, width: Width) -> Border {
        Border::new(
            vec![
                BorderFragment::new(length - 1, width, Width::None, width),
                BorderFragment::new(1, Width::None, Width::None, width),
            ],
            length,
        )
    }

    pub fn default_top(length: usize, width: Width) -> Border {
        Border::default_left(length, width)
    }

    pub fn default_right(length: usize, width: Width) -> Border {
        Border::new(
            vec![
                BorderFragment::new(length - 1, width, width, Width::None),
                BorderFragment::new(1, Width::None, width, Width::None),
            ],
            length,
        )
    }

    pub fn default_bottom(length: usize, width: Width) -> Border {
        Border::default_right(length, width)
    }

    pub fn add_after(&self, other: &Border) -> Border {
        let mut fragments: Vec<BorderFragment> = self.fragments.clone();
        let last_old = fragments.last_mut().unwrap();
        let first_new = &other.fragments[0];

        *last_old = BorderFragment::new(
            first_new.length,
            *last_old.width.combine(&first_new.width),
            *last_old.left_start.combine(&first_new.left_start),
            *last_old.right_start.combine(&first_new.right_start),
        );

        fragments.extend_from_slice(&other.fragments[1..]);
        Border::new(fragments, self.length + other.length - 1)
    }

    pub fn combine<'a>(&'a self, other: &'a Border) -> Border {
        if self.length != other.length {
            panic!("Border: cannot combine two borders of different lengths");
        }

        struct Combination<'a> {
            forward: &'a Width,
            left: &'a Width,
            right: &'a Width,
        }

        let mut chars: Vec<Combination> = Vec::with_capacity(self.length);
        for BorderFragment {
            length,
            width,
            left_start,
            right_start,
        } in self.fragments.iter()
        {
            chars.push(Combination {
                forward: width,
                left: left_start,
                right: right_start,
            });
            for _ in 1..*length {
                chars.push(Combination {
                    forward: width,
                    left: &Width::None,
                    right: &Width::None,
                });
            }
        }

        let mut i: usize = 0;
        for BorderFragment {
            length,
            width,
            left_start,
            right_start,
        } in other.fragments.iter()
        {
            chars[i].forward = chars[i].forward.combine(width);
            chars[i].left = chars[i].left.combine(left_start);
            chars[i].right = chars[i].right.combine(right_start);
            i += 1;

            for _ in 1..*length {
                chars[i].forward = chars[i].forward.combine(width);
                i += 1;
            }
        }

        let mut fragments: Vec<BorderFragment> =
            Vec::with_capacity(self.fragments.len() + other.fragments.len());

        let mut prev: &'a Width = chars[0].forward;
        for ch in chars {
            match ch {
                Combination {
                    forward,
                    left: &Width::None,
                    right: &Width::None,
                } if *forward == *prev => {
                    fragments.last_mut().unwrap().length += 1;
                }
                _ => {
                    fragments.push(BorderFragment::new(1, *ch.forward, *ch.left, *ch.right));
                    prev = ch.forward;
                }
            }
        }

        Border::new(fragments, self.length)
    }

    pub fn render_view(&self, orientation: Orientation) -> String {
        let mut view = String::with_capacity(self.length);
        let mut prev: &Width = &Width::None;
        for fragment in self.fragments.iter() {
            view.push_str(&fragment.render_view(prev, orientation));
            prev = &fragment.width;
        }

        view
    }
}

pub struct CellBorder {
    left: Border,
    right: Border,
    top: Border,
    bottom: Border,

    height: usize,
    width: usize,
}

impl CellBorder {
    pub fn new(left: Border, right: Border, top: Border, bottom: Border) -> CellBorder {
        if left.length != right.length || top.length != bottom.length {
            panic!(
                "CellBorder: Inconsistent border length. Top: {}, bottom: {}, left: {}, right: {}",
                top.length, bottom.length, left.length, right.length
            );
        }

        let height = left.length;
        let width = top.length;
        CellBorder {
            left,
            right,
            top,
            bottom,
            height,
            width,
        }
    }

    pub fn atomic(height_pt: usize, width_pt: usize, width: Width) -> CellBorder {
        CellBorder::new(
            Border::default_left(height_pt, width),
            Border::default_right(height_pt, width),
            Border::default_top(width_pt, width),
            Border::default_bottom(width_pt, width),
        )
    }

    pub fn check_size(&self, height: usize, width: usize) -> bool {
        self.height == height && self.width == width
    }

    pub fn add_horizontal(&self, other: &CellBorder) -> (CellBorder, Border) {
        (
            CellBorder::new(
                self.left.clone(),
                other.right.clone(),
                self.top.add_after(&other.top),
                self.bottom.add_after(&other.bottom),
            ),
            self.right.combine(&other.left),
        )
    }

    pub fn add_vertical(&self, other: &CellBorder) -> (CellBorder, Border) {
        (
            CellBorder::new(
                self.left.add_after(&other.left),
                self.right.add_after(&other.right),
                self.top.clone(),
                other.bottom.clone(),
            ),
            self.bottom.combine(&other.top),
        )
    }

    pub fn combine(&self, other: &CellBorder) -> CellBorder {
        if self.height != other.height || self.width != other.width {
            panic!("CellBorder: cannot combine two cells of different size. Self: height {}, width {}. Other: height {}, width {}",
                   self.height, self.width, other.height, other.width);
        }

        CellBorder::new(
            self.left.combine(&other.left),
            self.right.combine(&other.right),
            self.top.combine(&other.top),
            self.bottom.combine(&other.bottom),
        )
    }

    pub fn render_view(&self, text: &Vec<String>) -> Vec<String> {
        let mut textbox: Vec<String> = Vec::with_capacity(self.height);
        textbox.push(self.top.render_view(Orientation::Horizontal));

        let left_border = self.left.render_view(Orientation::Vertical);
        let right_border = self.right.render_view(Orientation::Vertical);

        for ((line, left), right) in text
            .iter()
            .zip(left_border.chars().skip(1))
            .zip(right_border.chars().skip(1))
        {
            textbox.push(format!("{}{}{}", left, line, right));
        }

        textbox.push(self.bottom.render_view(Orientation::Horizontal));
        textbox
    }

    pub fn render_view_empty(&self) -> Vec<String> {
        self.render_view(&vec![" ".repeat(self.width - 2); self.height - 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fragment_created() {
        let fragment = BorderFragment::new(5, Width::Light, Width::None, Width::None);
        assert_eq!(5, fragment.length);
        assert_eq!(
            "╶────",
            fragment.render_view(&Width::None, Orientation::Horizontal)
        );
        assert_eq!(
            "╷││││",
            fragment.render_view(&Width::None, Orientation::Vertical)
        );
    }

    #[test]
    fn fragment_start_pieces() {
        let fragment = BorderFragment::new(5, Width::Light, Width::Light, Width::Heavy);
        assert_eq!(
            "╅────",
            fragment.render_view(&Width::Heavy, Orientation::Horizontal)
        );
        assert_eq!(
            "╄││││",
            fragment.render_view(&Width::Heavy, Orientation::Vertical)
        );
    }
}
