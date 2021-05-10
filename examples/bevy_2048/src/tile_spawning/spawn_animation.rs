//! This module contains the implementation of the SpawnAnimation component and
//! its system.
use crate::common::{Animation, GameSize};
use bevy::prelude::*;

/// Component used to animate the tiles spawning.
pub struct SpawnAnimation {
    pub animation: Animation,
}

impl Default for SpawnAnimation {
    /// Sets the animation to finish after 3 updates.
    fn default() -> Self {
        Self {
            animation: Animation::new(3),
        }
    }
}

/// Animating each tile that contains SpawnAnimation component.
/// When the animation is finished, the SpawnAnimation component
/// is removed from the entity.
pub fn spawn_animation(
    mut commands: Commands,
    time: Res<Time>,
    game_size: Res<GameSize>,
    entity: Entity,
    mut spawn_anim: Mut<SpawnAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if spawn_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = game_size.tile_size() * spawn_anim.animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the component is being removed.
    if spawn_anim.animation.finished() {
        commands.remove_one::<SpawnAnimation>(entity);
    }
}
