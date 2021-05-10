//! This module contains the implementation of the MergeAnimation component and
//! system.

use crate::common::{Animation, GameSize};
use bevy::prelude::*;

/// Component used to animate the tiles that have been merged.
pub struct MergeAnimation {
    pub animation: Animation,
}

impl Default for MergeAnimation {
    /// Sets the animation to finish after 8
    fn default() -> Self {
        let func = |x| 4.0 * x * (1.0 - x);

        Self {
            animation: Animation::with_func(8, func),
        }
    }
}

/// Animating all the tiles that have been merged.
pub fn merge_animation(
    mut commands: Commands,
    time: Res<Time>,
    game_size: Res<GameSize>,
    entity: Entity,
    mut merge_anim: Mut<MergeAnimation>,
    mut sprite: Mut<Sprite>,
) {
    if merge_anim.animation.update(time.delta_seconds) {
        // Updating the sprite size while the animation is not finished.
        let size = game_size.tile_size()
            + game_size.merge_size() * merge_anim.animation.value();
        sprite.size.set_x(size);
        sprite.size.set_y(size);
    }

    // When the animation is finished, the component is being removed.
    if merge_anim.animation.finished() {
        commands.remove_one::<MergeAnimation>(entity);
    }
}
