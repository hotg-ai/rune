//! This module contains the implementation of the 'how to' text.
use bevy::prelude::*;

use super::RightSideNode;

const EXPLANATION_TEXT: &str = r#"Use arrow keys or
WASD keys to merge
the tiles with the
same color. Press
SPACE to restart."#;

/// Spawns the 'how to' text.
pub fn spawn_how_to_node(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rs_node_entity: Entity,
    _: &RightSideNode,
) {
    let font_handle = assets.get_handle("fonts/FiraSans-Bold.ttf");

    commands
        // Base node.
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(90.0), Val::Percent(70.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                margin: Rect::all(Val::Percent(5.0)),
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(40, 40, 40).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            // Creates a new text for every line (in order to align the whole text to the middle).
            for line in EXPLANATION_TEXT.lines().rev() {
                spawn_text(parent, line, 25.0, font_handle.clone());
            }

            // Title.
            spawn_text(parent, "How to play:", 40.0, font_handle);
        });

    // Making 'how to' text as a child of the left side node.
    commands.push_children(rs_node_entity, &[commands.current_entity().unwrap()]);
}

/// Creates a text as a child of a given parent.
fn spawn_text(parent: &mut ChildBuilder, text: &str, font_size: f32, font_handle: Handle<Font>) {
    parent
        // Base node.
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(font_size)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            draw: Draw {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Adding the text as a child.
            parent.spawn(TextComponents {
                style: Style::default(),
                text: Text {
                    value: text.to_string(),
                    font: font_handle,
                    style: TextStyle {
                        font_size,
                        color: Color::WHITE,
                    },
                },
                ..Default::default()
            });
        });
}
