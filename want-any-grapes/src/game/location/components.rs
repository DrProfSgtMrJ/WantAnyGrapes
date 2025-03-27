use bevy::prelude::*;

use crate::game::money::components::CurrencyType;

#[derive(Debug, Component)]
pub struct Location {
    pub display_name: String,
}

#[derive(Debug, Bundle)]
pub struct LocationBundle {
    pub location: Location,
    pub currency_type: CurrencyType
}