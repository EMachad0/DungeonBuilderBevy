use bevy::prelude::IVec2;

use crate::orientation::Orientation;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Door {
    pub position: IVec2,
    pub orientation: Orientation,
}

impl Door {
    pub fn new<T: Into<IVec2>>(position: T, orientation: Orientation) -> Self {
        Self {
            position: position.into(),
            orientation,
        }
    }
}
