use bevy::prelude::*;

pub const NO_PADDING: UiRect = UiRect {
    left: Val::Px(0.0),
    right: Val::Px(0.0),
    top: Val::Px(0.0),
    bottom: Val::Px(0.0),
};
pub const NO_BORDER: UiRect = NO_PADDING;