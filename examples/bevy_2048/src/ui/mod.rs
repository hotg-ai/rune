mod left_side;
mod right_side;

use bevy::prelude::*;
use left_side::LeftSidePlugin;
use right_side::RightSidePlugin;

static ROOT_CREATION_STAGE: &str = "ROOT-CREATION";
static POST_ROOT_CREATION_STAGE: &str = "POST-ROOT-CREATION";
/// This plugin builds the ui system into the app.
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_startup_stage(ROOT_CREATION_STAGE)
            .add_startup_stage_after(ROOT_CREATION_STAGE, POST_ROOT_CREATION_STAGE)
            .add_startup_system_to_stage(ROOT_CREATION_STAGE, create_root.system())
            // Should be added after the stages have been added.
            .add_plugin(LeftSidePlugin)
            .add_plugin(RightSidePlugin);
    }
}

pub struct RootNode;

fn create_root(mut commands: Commands, assets: Res<AssetServer>, mut _fonts: ResMut<Assets<Font>>) {
    // Loading the font once.
    assets.load::<Font, _>("fonts/FiraSans-Bold.ttf");

    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        // root node
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(RootNode);
}
