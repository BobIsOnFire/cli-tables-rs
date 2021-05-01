#![allow(dead_code)]

mod width;
pub use width::Width;

mod border;
pub use border::{Border, CellBorder, Orientation};

// TODO panic tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::borders::Orientation::*;
    use crate::borders::Width::*;
    #[test]
    fn border_created() {
        let border = Border::default_top(10, Heavy);
        assert_eq!("┏━━━━━━━━┓", border.render_view(Horizontal));
    }

    #[test]
    fn border_add_after() {
        let border1 = Border::default_top(10, Heavy);
        let border2 = Border::default_bottom(5, Light);
        let border = border1.add_after(&border2);
        assert_eq!("┏━━━━━━━━╅───┘", border.render_view(Horizontal));
    }

    #[test]
    fn border_combine() {
        let border1 = Border::default_top(10, Heavy);
        let border2 = Border::default_top(5, Light);
        let border3 = Border::default_bottom(7, Light);
        let border4 = Border::default_bottom(8, Heavy);

        let combine_border1 = border1.add_after(&border2);
        assert_eq!("┏━━━━━━━━┱───┐", combine_border1.render_view(Horizontal));

        let combine_border2 = border3.add_after(&border4);
        assert_eq!("└─────┺━━━━━━┛", combine_border2.render_view(Horizontal));

        let final_border = combine_border1.combine(&combine_border2);
        assert_eq!("┢━━━━━┻━━┳━━━┩", final_border.render_view(Horizontal));
    }

    #[test]
    fn block_consumes_everything() {
        let border1 = Border::default_top(4, Light);
        let border2 = Border::default_top(6, Block);
        let border3 = Border::default_top(6, Light);
        let border4 = Border::default_bottom(7, Light);
        let border5 = Border::default_bottom(8, Light);

        let combine_border1 = border1.add_after(&border2).add_after(&border3);
        let combine_border2 = border4.add_after(&border5);

        let final_border = combine_border1.combine(&combine_border2);
        assert_eq!("├──██████────┤", final_border.render_view(Horizontal));
    }

    #[test]
    fn full_border() {
        let cell = CellBorder::atomic(5, 15, Heavy);
        assert_eq!(
            vec![
                "┏━━━━━━━━━━━━━┓",
                "┃             ┃",
                "┃             ┃",
                "┃             ┃",
                "┗━━━━━━━━━━━━━┛"
            ],
            cell.render_view_empty()
        );
    }

    #[test]
    fn cell_combination_horizontal() {
        let cell1 = CellBorder::atomic(5, 15, Heavy);
        let cell2 = CellBorder::atomic(5, 10, Light);
        let (cell, border) = cell1.add_horizontal(&cell2);
        assert_eq!("┱┃┃┃┹", border.render_view(Vertical));
        assert_eq!(
            vec![
                "┏━━━━━━━━━━━━━┱────────┐",
                "┃                      │",
                "┃                      │",
                "┃                      │",
                "┗━━━━━━━━━━━━━┹────────┘"
            ],
            cell.render_view_empty()
        );
    }

    #[test]
    fn cell_combination_vertical() {
        let cell1 = CellBorder::atomic(5, 15, Light);
        let cell2 = CellBorder::atomic(6, 15, Heavy);
        let (cell, border) = cell1.add_vertical(&cell2);
        assert_eq!("┢━━━━━━━━━━━━━┪", border.render_view(Horizontal));
        assert_eq!(
            vec![
                "┌─────────────┐",
                "│             │",
                "│             │",
                "│             │",
                "┢             ┪",
                "┃             ┃",
                "┃             ┃",
                "┃             ┃",
                "┃             ┃",
                "┗━━━━━━━━━━━━━┛"
            ],
            cell.render_view_empty()
        );
    }

    #[test]
    fn cell_combination_complex() {
        let cell1 = CellBorder::atomic(5, 8, Light);
        let cell2 = CellBorder::atomic(5, 8, Heavy);
        let cell3 = CellBorder::atomic(6, 15, Light);

        let (cell_complex, border) = cell1.add_horizontal(&cell2);
        assert_eq!("┲┃┃┃┺", border.render_view(Vertical));

        let (cell_final, border) = cell_complex.add_vertical(&cell3);
        assert_eq!("├──────┺━━━━━━┩", border.render_view(Horizontal));

        assert_eq!(
            vec![
                "┌──────┲━━━━━━┓",
                "│             ┃",
                "│             ┃",
                "│             ┃",
                "├             ┩",
                "│             │",
                "│             │",
                "│             │",
                "│             │",
                "└─────────────┘"
            ],
            cell_final.render_view_empty()
        );
    }
}
