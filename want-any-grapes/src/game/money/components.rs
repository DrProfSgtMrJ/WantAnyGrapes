use bevy::prelude::*;



#[derive(Debug, Component, Default)]
pub struct Money {
    pub amount: f64
}

#[derive(Debug, Component)]
pub enum CurrencyType {
    Dollar,
    Euro,
    // TODO: Add more currency types
}

#[derive(Debug, Component)]
pub struct ConversionRate {
    pub rate: f64
}