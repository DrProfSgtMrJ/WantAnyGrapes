use bevy::prelude::*;
mod ui;

use ui::{menu::main_menu::systems::setup_main_menu, setup_ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, setup_main_menu)
        .run();
}