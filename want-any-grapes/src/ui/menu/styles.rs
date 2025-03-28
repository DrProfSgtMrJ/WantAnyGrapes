use bevy::prelude::*;

pub const PRIMARY_BUTTON_WIDTH: f32 = 300.0;
pub const PRIMARY_BUTTON_HEIGHT: f32 = 65.0;
pub const PRIMARY_BUTTON_MARGIN: f32 = 20.0;

// TODO: highlight, clicked, disabled colors

pub const PRIMARY_BACKGROUND_COLOR: Color = Color::linear_rgb(1.0, 0.9176, 0.6549); // #ffeaa7

pub const PRIMARY_BUTTON_COLOR: Color = Color::linear_rgb(0.9922, 0.9059, 0.4157); // #fde769

pub const SECONDARY_BUTTON_COLOR: Color = Color::linear_rgb(0.7725, 0.8824, 0.3882); // #c5e163

pub const TERTIARY_BUTTON_COLOR: Color = Color::linear_rgb(0.4627, 0.5882, 0.6510); // #7696a6

pub const BUTTON_BORDER_COLOR: Color = Color::linear_rgb(0.1490, 0.2745, 0.3255); // #264653

pub const BUTTON_TEXT_COLOR: Color = Color::linear_rgb(0.1490, 0.2745, 0.3255); // #264653

pub fn primary_button_node() -> Node {
    Node {
        width: Val::Px(PRIMARY_BUTTON_WIDTH),
        height: Val::Px(PRIMARY_BUTTON_HEIGHT),
        border: UiRect::all(Val::Px(2.0)),
        margin: UiRect::all(Val::Px(PRIMARY_BUTTON_MARGIN)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}