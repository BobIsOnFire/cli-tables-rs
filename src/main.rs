#[macro_use]
extern crate cli_tables;

use cli_tables::{cells::Table};

fn main() {
    let table: Table<_> = col![
        row![
            textcell![1, {padding = 3, span_width = 2, border = Block}],
            textcell![1, {pt_height = 10}],
        ],
        row![{border = Heavy},
            textcell![1, {alignment = Right}],
            textcell![1, {alignment = Left}],
            textcell![1, {pt_width = 10}],
        ],
    ].into();

    table.print();
}
