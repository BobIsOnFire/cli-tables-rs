#![allow(dead_code)]
#![allow(unused_macros)]

#[macro_use]
mod cell;
pub use cell::{Alignment, Cell, CellConfig, CellView};

#[macro_use]
mod text_cell;
pub use text_cell::TextCell;

#[macro_use]
mod table;
pub use table::{Row, Table};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borders::Width::*;
    #[test]
    fn textcell_draw_simple() {
        let cell = textcell!("Hello!", { width = Heavy });

        assert_eq!(
            vec!["┏━━━━━━━━━┓", "┃ Hello!  ┃", "┗━━━━━━━━━┛"],
            cell.draw(1, 9).complete()
        )
    }

    #[test]
    fn textcell_draw_multiple_lines() {
        let cell = textcell!("Hello!");
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec!["         ", " Hello!  ", "         "], text);
    }

    #[test]
    fn textcell_draw_wrap_line() {
        let cell = textcell!("Hello! My name is CLI Table");
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec!["Hello! My", "name is  ", "CLI Table"], text);
    }

    #[test]
    fn textcell_draw_wrap_word() {
        let cell = textcell!("Supadupaliciousities");
        let (text, _) = cell.draw(3, 9).unwrap();
        assert_eq!(vec!["Supadupal", "iciousiti", "es       "], text);
    }

    #[test]
    fn textcell_draw_cut_text() {
        let cell = textcell!("Supadupaliciousities");
        let (text, _) = cell.draw(1, 9).unwrap();
        assert_eq!(vec!["Supadupal"], text);
    }

    #[test]
    fn row_draw_line() {
        let row = row![
            textcell!("When I close my eyes, I cannot see."),
            textcell!("Oh man..."),
            textcell!("Call me Ishmael.")
        ];

        let (text, border) = row.draw(3, &vec![12, 5, 4]).unwrap();
        assert_eq!(
            vec![
                "When I close│Oh   │Call",
                "my eyes, I  │man..│me  ",
                "cannot see. │.    │Ishm"
            ],
            text
        );
        assert_eq!(
            vec![
                "┌────────────┬─────┬────┐",
                "│When I close│Oh   │Call│",
                "│my eyes, I  │man..│me  │",
                "│cannot see. │.    │Ishm│",
                "└────────────┴─────┴────┘"
            ],
            border.render_view(&text)
        );
    }

    #[test]
    fn table_draw() {
        let table = table![
            row![
                textcell!("When I close my eyes, I cannot see."),
                textcell!("Oh man..."),
                textcell!("Call me Ishmael.")
            ],
            row![textcell!(1), textcell!(33), textcell!(135)]
        ];
        assert_eq!(
            vec![
                "┌──────────────────────┬─────┬───────────┐",
                "│When I close my eyes, │Oh   │Call me    │",
                "│I cannot see.         │man..│Ishmael.   │",
                "│                      │.    │           │",
                "├──────────────────────┼─────┼───────────┤",
                "│          1           │ 33  │    135    │",
                "└──────────────────────┴─────┴───────────┘"
            ],
            table.draw(0, 40).complete()
        );
    }

    #[test]
    fn table_draw_small() {
        let table = table![
            row![
                { width = Heavy },
                textcell!(10, { padding = 1 }),
                textcell!(3, { padding = 1 }),
                textcell!(0, { padding = 1 })
            ],
            row![
                textcell!(1, { padding = 1 }),
                textcell!(33),
                textcell!(135, { padding = 1 })
            ]
        ];

        assert_eq!(
            vec![
                "┏━━━━┯━━━┯━━━━━┓",
                "┃ 10 │ 3 │  0  ┃",
                "┡━━━━┿━━━┿━━━━━┩",
                "│ 1  │33 │ 135 │",
                "└────┴───┴─────┘"
            ],
            table.draw(0, 0).complete()
        );
    }

    #[test]
    fn table_draw_inside_table() {
        let table = table![
            row![
                textcell!(10, {width = Heavy, padding = 1}),
                table![
                    row![textcell!(2, {width = Heavy, padding = 1})],
                    row![textcell!(3, { padding = 1 })]
                ],
                textcell!(3)
            ],
            row![
                textcell!(1),
                textcell!(33),
                textcell!(135, { width = Heavy })
            ]
        ];
        assert_eq!(
            vec![
                "┏━━━━┳━━━┱───┐",
                "┃    ┃ 2 ┃   │",
                "┃ 10 ┣━━━┩ 3 │",
                "┃    ┃ 3 │   │",
                "┡━━━━╃───╆━━━┪",
                "│ 1  │33 ┃135┃",
                "└────┴───┺━━━┛"
            ],
            table.draw(0, 0).complete()
        );
    }
}
