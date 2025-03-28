pub mod menu;

use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
) {
    commands.spawn(Camera2d::default());
}