use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct OnMainMenuScreen {}

#[derive(Debug, Component)]
pub enum MenuButtonAction {
    LogIn,
    CreateAccount,
    Settings,
    Quit,
}

#[derive(Debug, Component)]
pub struct MainMenu {}

#[derive(Debug, Component)]
pub struct LogInButton {}

#[derive(Debug, Component)]
pub struct SignUpButton {}

#[derive(Debug, Component)]
pub struct SettingsButton {}

#[derive(Debug, Component)]
pub struct QuitButton {}