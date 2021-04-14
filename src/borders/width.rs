// N = None, L = Light, H = Heavy, B = Block
static BOX_DRAWING_MAPPER: [char; 256] = [
    /*                  || up:N, left:      || up:L, left:      || up:H, left:      || up:B, left:      || */
    /*                  || N || L || H || B || N || L || H || B || N || L || H || B || N || L || H || B || */
    /* down:N, right:N */ ' ', '╴', '╸', '█', '╵', '┘', '┙', '█', '╹', '┚', '┛', '█', '█', '█', '█', '█',
    /* down:L, right:N */ '╷', '┐', '┑', '█', '│', '┤', '┥', '█', '╿', '┦', '┩', '█', '█', '█', '█', '█',
    /* down:H, right:N */ '╻', '┒', '┓', '█', '╽', '┧', '┪', '█', '┃', '┨', '┫', '█', '█', '█', '█', '█',
    /* down:B, right:N */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:N, right:L */ '╶', '─', '╾', '█', '└', '┴', '┵', '█', '┖', '┸', '┹', '█', '█', '█', '█', '█',
    /* down:L, right:L */ '┌', '┬', '┭', '█', '├', '┼', '┽', '█', '┞', '╀', '╃', '█', '█', '█', '█', '█',
    /* down:H, right:L */ '┎', '┰', '┱', '█', '┟', '╁', '╅', '█', '┠', '╂', '╉', '█', '█', '█', '█', '█',
    /* down:B, right:L */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:N, right:H */ '╺', '╼', '━', '█', '┕', '┶', '┷', '█', '┗', '┺', '┻', '█', '█', '█', '█', '█',
    /* down:L, right:H */ '┍', '┮', '┯', '█', '┝', '┾', '┿', '█', '┡', '╄', '╇', '█', '█', '█', '█', '█',
    /* down:H, right:H */ '┏', '┲', '┳', '█', '┢', '╆', '╈', '█', '┣', '╊', '╋', '█', '█', '█', '█', '█',
    /* down:B, right:H */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:N, right:B */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:L, right:B */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:H, right:B */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
    /* down:B, right:B */ '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█', '█',
];

#[derive(Clone, Copy, PartialEq)]
pub enum Width {
    None,
    Light,
    Heavy,
    Block
}

impl Width {
    pub fn combine<'a>(&'a self, other: &'a Width) -> &'a Width {
        match (self, other) {
            (Width::None, Width::None) => &Width::None,
            (Width::None, _) => other,
            (_, Width::None) => self,
            (Width::Light, _) => other,
            (_, Width::Light) => self,
            (Width::Heavy, _) => other,
            (_, Width::Heavy) => self,
            _ => other
        }
    }

    fn index(&self) -> usize {
        match self {
            Width::None => 0,
            Width::Light => 1,
            Width::Heavy => 2,
            Width::Block => 3
        }
    }

    pub fn draw_char(top: &Width, left: &Width, bottom: &Width, right: &Width) -> char {
        BOX_DRAWING_MAPPER[
            ((right.index() * 4 + bottom.index()) * 4
            + top.index()) * 4 + left.index()
        ]
    }
}
