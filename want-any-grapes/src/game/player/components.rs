use bevy::prelude::*;

use crate::game::{location::components::*, money::components::Money};

#[derive(Debug, Component)]
pub struct Player {
    pub display_name: String
}

#[derive(Debug)]
pub enum FarmerType {
    Lemon,
    Sugar
}

#[derive(Debug)]
pub enum WholesalerType {
    Lemon,
    Sugar
}

#[derive(Debug, Component)]
pub enum PlayerRole {
    Farmer { farmer_type: FarmerType },
    Wholesaler { wholesaler_type: WholesalerType },
    BusinessOwner
}

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub player_role: PlayerRole,
    pub location: LocationBundle,
    pub money: Money,
    pub avatar: Sprite,
}