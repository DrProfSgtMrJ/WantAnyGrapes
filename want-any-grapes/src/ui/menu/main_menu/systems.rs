use bevy::prelude::*;

use crate::ui::menu::styles::{primary_button_node, BUTTON_BORDER_COLOR, BUTTON_TEXT_COLOR, PRIMARY_BACKGROUND_COLOR, PRIMARY_BUTTON_COLOR, SECONDARY_BUTTON_COLOR, TERTIARY_BUTTON_COLOR};

use super::components::{MenuButtonAction, OnMainMenuScreen};

pub fn setup_main_menu(
    mut commands: Commands,
    //_asset_server: Res<AssetServer>,
) {
    let button = primary_button_node();
    // Create background node
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }, 
        BackgroundColor(PRIMARY_BACKGROUND_COLOR),
        OnMainMenuScreen {},
    )).with_children(|parent| {
    // Display four buttons for each action available from the main menu:
        // - log in 
        // - sign up
        // - settings
        // - quit
        parent
            .spawn((
                Button,
                button.clone(),
                BackgroundColor(PRIMARY_BUTTON_COLOR),
                BorderColor(BUTTON_BORDER_COLOR),
                BorderRadius::all(Val::Px(10.0)),
                MenuButtonAction::LogIn,
            ))
            .with_children(|parent| {
                parent.spawn( (
                        Text::new("Log In"),
                        TextColor(BUTTON_TEXT_COLOR),
                    ));
            });
    });
}