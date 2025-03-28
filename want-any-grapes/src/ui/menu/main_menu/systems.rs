use bevy::prelude::*;

use crate::ui::menu::styles::{primary_button_node, BUTTON_TEXT_COLOR, PRIMARY_BACKGROUND_COLOR, PRIMARY_BUTTON_COLOR, SECONDARY_BUTTON_COLOR, TERTIARY_BUTTON_COLOR};

use super::components::{MenuButtonAction, OnMainMenuScreen};

pub fn setup_main_menu(
    mut commands: Commands,
    //_asset_server: Res<AssetServer>,
) {
    // button
    let button_node = primary_button_node();
    let button_text_font = TextFont {
        font_size: 30.0,
        ..default()
    };
    commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                OnMainMenuScreen {},
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(PRIMARY_BACKGROUND_COLOR.into()),
                    ))
                    .with_children(|parent| {
                        // Display the game name
                        parent.spawn((
                            Text::new("Want Any Grapes?"),
                            TextFont {
                                font_size: 67.0,
                                ..default()
                            },
                            TextColor(BUTTON_TEXT_COLOR.into()),
                            Node {
                                margin: UiRect::all(Val::Px(50.0)),
                                ..default()
                            },
                        ));

                        // Display four buttons for each action available from the main menu:
                        // - log in
                        // - create account
                        // - settings
                        // - quit (todo)
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(PRIMARY_BUTTON_COLOR.into()),
                                MenuButtonAction::LogIn,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Log In"),
                                    button_text_font.clone(),
                                    TextColor(PRIMARY_BUTTON_COLOR.into()),
                                ));
                            });
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(SECONDARY_BUTTON_COLOR.into()),
                                MenuButtonAction::CreateAccount,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Create Account"),
                                    button_text_font.clone(),
                                    TextColor(BUTTON_TEXT_COLOR.into()),
                                ));
                            });
                        parent
                            .spawn((
                                Button,
                                button_node.clone(),
                                BackgroundColor(TERTIARY_BUTTON_COLOR.into()),
                                MenuButtonAction::Settings,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    Text::new("Settings"),
                                    button_text_font,
                                    TextColor(BUTTON_TEXT_COLOR.into()),
                                ));
                            });
                    });
            });

}