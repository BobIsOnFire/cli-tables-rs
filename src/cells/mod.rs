#![allow(dead_code)]

mod cell;
pub use cell::{Cell, CellView};

mod text_cell;
pub use text_cell::{Alignment, TextCell};

mod table;
pub use table::{Row, Table};

#[cfg(test)]
mod tests {
    use crate::borders::Width;
    use super::*;
    #[test]
    fn textcell_draw_simple() {
        let cell = TextCell::default(String::from("Hello!"), 0, Width::Heavy);
        assert_eq!(vec![
            "┏━━━━━━━━━┓",
            "┃ Hello!  ┃",
            "┗━━━━━━━━━┛"
        ], cell.draw(1, 9).complete())
    }

    #[test]
    fn textcell_draw_multiple_lines() {
        let cell = TextCell::default(String::from("Hello!"), 0, Width::None);
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec![
            "         ",
            " Hello!  ",
            "         "
        ], text);
    }

    #[test]
    fn textcell_draw_wrap_line() {
        let cell = TextCell::default(String::from("Hello! My name is CLI Table"), 0, Width::None);
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec![
            "Hello! My",
            "name is  ",
            "CLI Table"
        ], text);
    }

    #[test]
    fn textcell_draw_wrap_word() {
        let cell = TextCell::default(String::from("Supadupaliciousities"), 0, Width::None);
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec![
            "Supadupal",
            "iciousiti",
            "es       "
        ], text);
    }

    #[test]
    fn textcell_draw_cut_text() {
        let cell = TextCell::default(String::from("Supadupaliciousities"), 0, Width::None);
        let (text, _) = cell.draw(1, 9).unwrap();
        assert_eq!(vec!["Supadupal"], text);
    }

    #[test]
    fn row_draw_line() {
        let row = Row::default(vec![
            Box::new(TextCell::default(String::from("When I close my eyes, I cannot see."), 0, Width::Light)),
            Box::new(TextCell::default(String::from("Oh man..."), 0, Width::Light)),
            Box::new(TextCell::default(String::from("Call me Ishmael."), 0, Width::Light)),
        ]);
        let (text, border) = row.draw(3, &vec![12, 5, 4]).unwrap();
        assert_eq!(vec![
            "When I close│Oh   │Call",
            "my eyes, I  │man..│me  ",
            "cannot see. │.    │Ishm"
        ], text);
        assert_eq!(vec![
            "┌────────────┬─────┬────┐",
            "│When I close│Oh   │Call│",
            "│my eyes, I  │man..│me  │",
            "│cannot see. │.    │Ishm│",
            "└────────────┴─────┴────┘"
        ], border.render_view(&text));
    }

    #[test]
    fn table_draw() {
        let row1 = Row::default(vec![
            Box::new(TextCell::default(String::from("When I close my eyes, I cannot see."), 0, Width::Light)),
            Box::new(TextCell::default(String::from("Oh man..."), 0, Width::Light)),
            Box::new(TextCell::default(String::from("Call me Ishmael."), 0, Width::Light)),
        ]);
        let row2 = Row::default(vec![
            Box::new(TextCell::default(String::from("1"), 0, Width::Light)),
            Box::new(TextCell::default(String::from("33"), 0, Width::Light)),
            Box::new(TextCell::default(String::from("135"), 0, Width::Light))
        ]);

        let table = Table::default(vec![row1, row2]);
        assert_eq!(vec![
            "┌──────────────────────┬─────┬───────────┐",
            "│When I close my eyes, │Oh   │Call me    │",
            "│I cannot see.         │man..│Ishmael.   │",
            "│                      │.    │           │",
            "├──────────────────────┼─────┼───────────┤",
            "│          1           │ 33  │    135    │",
            "└──────────────────────┴─────┴───────────┘"
        ], table.draw(0, 40).complete());
    }

    #[test]
    fn table_draw_small() {
        let row1 = Row::header(vec![
            Box::new(TextCell::default(String::from("10"), 1, Width::Light)),
            Box::new(TextCell::default(String::from("3"), 1, Width::Light)),
            Box::new(TextCell::default(String::from("0"), 1, Width::Light)),
        ]);
        let row2 = Row::default(vec![
            Box::new(TextCell::default(String::from("1"), 1, Width::Light)),
            Box::new(TextCell::default(String::from("33"), 0, Width::Light)),
            Box::new(TextCell::default(String::from("135"), 1, Width::Light))
        ]);

        let table = Table::default(vec![row1, row2]);
        assert_eq!(vec![
            "┏━━━━┯━━━┯━━━━━┓",
            "┃ 10 │ 3 │  0  ┃",
            "┡━━━━┿━━━┿━━━━━┩",
            "│ 1  │33 │ 135 │",
            "└────┴───┴─────┘"
        ], table.draw(0, 0).complete());
    }

    #[test]
    fn table_draw_inside_table() {
        let inner_table = Table::default(vec![
            Row::default(vec![Box::new(TextCell::default(String::from("2"), 1, Width::Heavy))]),
            Row::default(vec![Box::new(TextCell::default(String::from("3"), 1, Width::Light))])
        ]);

        let row1 = Row::default(vec![
            Box::new(TextCell::default(String::from("10"), 1, Width::Heavy)),
            Box::new(inner_table),
            Box::new(TextCell::default(String::from("3"), 0, Width::Light)),
        ]);
        let row2 = Row::default(vec![
            Box::new(TextCell::default(String::from("1"), 0, Width::Light)),
            Box::new(TextCell::default(String::from("33"), 0, Width::Light)),
            Box::new(TextCell::default(String::from("135"), 0, Width::Heavy))
        ]);

        let table = Table::default(vec![row1, row2]);
        assert_eq!(vec![
            "┏━━━━┳━━━┱───┐",
            "┃    ┃ 2 ┃   │",
            "┃ 10 ┣━━━┩ 3 │",
            "┃    ┃ 3 │   │",
            "┡━━━━╃───╆━━━┪",
            "│ 1  │33 ┃135┃",
            "└────┴───┺━━━┛"
        ], table.draw(0, 0).complete());
    }
}
