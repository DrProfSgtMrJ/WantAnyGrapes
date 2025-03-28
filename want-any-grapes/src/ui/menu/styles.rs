use bevy::prelude::*;

pub const PRIMARY_BUTTON_WIDTH: f32 = 300.0;
pub const PRIMARY_BUTTON_HEIGHT: f32 = 65.0;
pub const PRIMARY_BUTTON_MARGIN: f32 = 20.0;

// TODO: highlight, clicked, disabled colors

// #ffeaa7
pub const PRIMARY_BACKGROUND_COLOR: Color = Color::linear_rgb(255.0, 234.0, 167.0);

// #fde769
pub const PRIMARY_BUTTON_COLOR: Color = Color::linear_rgb(253.0, 231.0, 106.0);

// #c5e163
pub const SECONDARY_BUTTON_COLOR: Color = Color::linear_rgb(197.0, 225.0, 99.0);

// #7696a6
pub const TERTIARY_BUTTON_COLOR: Color = Color::linear_rgb(118.0, 150.0, 166.0);

// #264653
pub const BUTTON_BORDER_COLOR: Color = Color::linear_rgb(38.0, 70.0, 83.0);

// #264653
pub const BUTTON_TEXT_COLOR: Color = Color::linear_rgb(38.0, 70.0, 83.0);


pub fn primary_button_node() -> Node {
    Node {
        width: Val::Px(PRIMARY_BUTTON_WIDTH),
        height: Val::Px(PRIMARY_BUTTON_HEIGHT),
        margin: UiRect::all(Val::Px(PRIMARY_BUTTON_MARGIN)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}