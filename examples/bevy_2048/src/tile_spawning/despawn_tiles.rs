//! This module contains the implementation of the Despawn component and its system.
use bevy::prelude::*;

use crate::{
    common::{Position, Tile},
    movement::{Merged, Moving},
};

use super::DespawnAnimation;

/// In order to despawn tiles properly, this component will remove
/// (with a system) all unusfull components and will add the
/// DespawnAnimation component.
pub struct Despawn;

/// This system removing all the unuseful components from a tile that should be despawned.
/// also adding the despawn animation.
pub fn despawn_tiles(mut commands: Commands, entity: Entity, _: &Despawn) {
    commands.remove_one::<Tile>(entity);
    commands.remove_one::<Position>(entity);
    commands.remove_one::<Despawn>(entity);
    commands.remove_one::<Option<Moving>>(entity);
    commands.remove_one::<Option<Merged>>(entity);
    commands.insert_one(entity, DespawnAnimation::default());
}
