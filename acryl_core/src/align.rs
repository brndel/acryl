use std::str::FromStr;

use crate::math::Unit;


#[derive(Debug)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl FromStr for MainAxisAlignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let align = match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "spaceBetween" => Self::SpaceBetween,
            _ => return Err(())
        };
        Ok(align)
    }
}

impl MainAxisAlignment {
    pub fn get_positions<T: Unit>(&self, max_space: T, spaces: &[T]) -> Vec<T> {
        let needed_space = spaces.iter().fold(T::from(0.0), |v, space| v + *space);
        let remaining_space = max_space - needed_space;

        let mut pos = match self {
            MainAxisAlignment::Start => T::from(0.0),
            MainAxisAlignment::Center => (remaining_space) / T::from(2.0),
            MainAxisAlignment::End => remaining_space,
            MainAxisAlignment::SpaceBetween => T::from(0.0),
        };

        let padding = match self {
            MainAxisAlignment::SpaceBetween => remaining_space / ((spaces.len() - 1) as f64).into(),
            _ => T::from(0.0),
        };

        let mut positions = Vec::new();

        for space in spaces {
            positions.push(pos);
            pos += *space + padding;
        }
        

        positions
    }
}

#[derive(Debug)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl FromStr for CrossAxisAlignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let align = match s {
            "start" => Self::Start,
            "center" => Self::Center,
            "end" => Self::End,
            "stretch" => Self::Stretch,
            _ => return Err(()),
        };
        Ok(align)
    }
}

impl CrossAxisAlignment {
    pub fn get_position<T: Unit>(&self, max_space: T, space: T) -> T {
        match self {
            CrossAxisAlignment::Center => (max_space - space) / T::from(2.0),
            CrossAxisAlignment::End => max_space - space,
            CrossAxisAlignment::Start => T::from(0.0),
            CrossAxisAlignment::Stretch => T::from(0.0),
        }
    }

    pub fn get_size<T: Unit>(&self, max_space: T, space: T) -> T {
        match self {
            CrossAxisAlignment::Start => space,
            CrossAxisAlignment::Center => space,
            CrossAxisAlignment::End => space,
            CrossAxisAlignment::Stretch => max_space,
        }
    }
}