use std::iter::FromIterator;

use crate::borders::Width;
use crate::config::utils::*;
use crate::config::{Alignment, Bound, CellBounds, UserProperties};

#[derive(Copy, Clone, Debug)]
pub struct CellConfig {
    pub border: Width,
    pub alignment: Alignment,
    pub padding: usize,

    pub bounds: CellBounds,
    pub span_height: usize,
    pub span_width: usize,
}

impl From<UserProperties> for CellConfig {
    fn from(props: UserProperties) -> Self {
        CellConfig {
            border: props.border,
            alignment: props.alignment,
            padding: props.padding,

            bounds: CellBounds::new(Bound::new(2, 2), Bound::new(props.pt_height + 1, props.pt_width + 1)),
            span_height: props.span_height,
            span_width: props.span_width,
        }
    }
}

impl Default for CellConfig {
    fn default() -> Self {
        Self::from(UserProperties::default())
    }
}

impl FromIterator<CellConfig> for Vertical<CellConfig> {
    fn from_iter<I: IntoIterator<Item = CellConfig>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(CellConfig {
            bounds: elems.iter().map(|x| x.bounds).collect::<Vertical<_>>().0,
            span_height: elems.iter().map(|x| x.span_height).sum(),
            span_width: elems.iter().map(|x| x.span_width).fold(1, |acc, x| lcm(acc, x)),
            ..CellConfig::default()
        })
    }
}

impl FromIterator<CellConfig> for Horizontal<CellConfig> {
    fn from_iter<I: IntoIterator<Item = CellConfig>>(iter: I) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self(CellConfig {
            bounds: elems.iter().map(|x| x.bounds).collect::<Horizontal<_>>().0,
            span_height: elems.iter().map(|x| x.span_height).fold(1, |acc, x| lcm(acc, x)),
            span_width: elems.iter().map(|x| x.span_width).sum(),
            ..CellConfig::default()
        })
    }
}
