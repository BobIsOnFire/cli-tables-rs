use crate::borders::Width;

#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    Default,
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn console(&self, multiline: bool) -> console::Alignment {
        match self {
            Alignment::Default => {
                if multiline {
                    console::Alignment::Left
                } else {
                    console::Alignment::Center
                }
            }
            Alignment::Left => console::Alignment::Left,
            Alignment::Center => console::Alignment::Center,
            Alignment::Right => console::Alignment::Right,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserProperties {
    pub border: Width,
    pub alignment: Alignment,
    pub padding: usize,

    pub pt_height: usize,
    pub pt_width: usize,

    pub span_height: usize,
    pub span_width: usize,
}

impl Default for UserProperties {
    fn default() -> Self {
        Self {
            border: Width::None,
            alignment: Alignment::Default,
            padding: 0,

            pt_height: 0,
            pt_width: 0,

            span_height: 1,
            span_width: 1,
        }
    }
}

#[macro_export]
macro_rules! do_properties {
    ($config:expr $(,)?) => { };
    ($config:expr, $field:ident=$value:expr $(,)?) => {
        $config.$field = $value;
    };

    ($config:expr, $field:ident=$value:expr, $($i:ident=$e:expr),+ $(,)?) => {
        do_properties!($config, $field=$value);
        do_properties!($config, $($i=$e),+);
    }
}

#[macro_export]
macro_rules! properties {
    ($($i:ident=$e:expr),* $(,)?) => {{
        use $crate::config::Alignment::*;
        use $crate::borders::Width::*;
        let mut props = $crate::config::UserProperties::default();
        do_properties!(props, $($i=$e),*);
        props
    }};
}
