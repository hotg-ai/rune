//! This module contains the implementation of `LeftSidePlugin`
//! and the system that creates the left side node.

mod new_game_button;
mod score_texts;

use bevy::prelude::*;

use super::{RootNode, POST_ROOT_CREATION_STAGE};
use new_game_button::NewGameButtonMaterials;

static POST_LS_CREATION_STAGE: &str = "POST-LEFT-SIDE-CREATION";

/// This plugin builds the left side ui into the app.
pub struct LeftSidePlugin;

impl Plugin for LeftSidePlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<NewGameButtonMaterials>()
            .add_startup_stage_after(POST_ROOT_CREATION_STAGE, POST_LS_CREATION_STAGE)
            .add_startup_system_to_stage(POST_ROOT_CREATION_STAGE, spawn_left_side_node.system())
            .add_startup_system_to_stage(POST_LS_CREATION_STAGE, score_texts::spawn_texts.system())
            .add_startup_system_to_stage(
                POST_LS_CREATION_STAGE,
                new_game_button::spawn_new_game_button.system(),
            )
            .add_system(new_game_button::new_game_button_system.system())
            .add_system(score_texts::score_text.system())
            .add_system(score_texts::highscore_text.system());
    }
}

/// An identifier for the left side node.
pub struct LeftSideNode;

/// Spawns the left side node as a child of the root node.
fn spawn_left_side_node(mut commands: Commands, root_entity: Entity, _: &RootNode) {
    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(20.0), Val::Percent(50.0)),
                align_self: AlignSelf::FlexEnd,
                align_items: AlignItems::FlexEnd,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(LeftSideNode);

    // Making the left side node as a child of the root node.
    commands.push_children(root_entity, &[commands.current_entity().unwrap()]);
}
