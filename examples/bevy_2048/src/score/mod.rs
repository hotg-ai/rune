//! This module contains the implementation of the components and the plugin for the score system.

use bevy::prelude::*;

mod highscore;
pub use highscore::HighScore;

/// This struct saves the score of the current game.
pub struct Score(pub u32);

/// This plugin builds the score system into the app.
pub struct ScoreSystemPlugin;

impl Plugin for ScoreSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<HighScore>()
            .add_resource(Score(0))
            .add_system(highscore::update_highscore.system());
    }
}
