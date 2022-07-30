//! This example has two text inputs: single-line and multiline

use bevy::prelude::*;

use bevy_slimy_widgets::text_input::{DefaultConstrains, TextCursorStyle, TextInputConstrains};
use bevy_slimy_widgets::{SlimyWidgetsPlugin, TextInputBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SlimyWidgetsPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            color: Color::GRAY.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // single-line input (up to 13 characters)
            parent.spawn_bundle(TextInputBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Px(30.0)),
                    margin: UiRect::all(Val::Auto),
                    border: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                placeholder: Text::from_section(
                    "Enter text...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::GRAY,
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                })
                .into(),
                text_style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                }
                .into(),
                text_alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Left,
                }
                .into(),
                color: Color::DARK_GRAY.into(),
                constrains: TextInputConstrains(vec![
                    Box::new(DefaultConstrains::MaxLength(13)),
                    Box::new(DefaultConstrains::DisallowedCharacters(vec!['\n'])),
                ]),
                cursor: TextCursorStyle::default(
                    24.0,
                    Color::WHITE.into(),
                    UiRect::all(Val::Undefined),
                    TextAlignment {
                        vertical: VerticalAlign::Top,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                ..Default::default()
            });

            // multiline input
            parent.spawn_bundle(TextInputBundle {
                style: Style {
                    size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                    margin: UiRect::all(Val::Auto),
                    border: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                placeholder: Text::from_section(
                    "Enter more text...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        color: Color::GRAY,
                    },
                )
                .with_alignment(TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                })
                .into(),
                text_style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    color: Color::WHITE,
                }
                .into(),
                text_alignment: TextAlignment {
                    vertical: VerticalAlign::Top,
                    horizontal: HorizontalAlign::Left,
                }
                .into(),
                color: Color::DARK_GRAY.into(),
                constrains: TextInputConstrains(Vec::new()),
                cursor: TextCursorStyle::default(
                    16.0,
                    Color::WHITE.into(),
                    UiRect::all(Val::Undefined),
                    TextAlignment {
                        vertical: VerticalAlign::Bottom,
                        horizontal: HorizontalAlign::Left,
                    },
                ),
                ..Default::default()
            });
        });
}
