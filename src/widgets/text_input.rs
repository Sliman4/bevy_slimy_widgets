use std::ops::Deref;
use std::time::Duration;

use ab_glyph::{Font as AbGlyphFont, FontArc, Glyph, PxScale, ScaleFont};
use bevy::app::EventReader;
use bevy::asset::Assets;
use bevy::core::{Time, Timer};
use bevy::ecs::component::Component;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::{ElementState, Input};
use bevy::prelude::{
    Added, BuildChildren, Changed, DespawnRecursiveExt, Entity, KeyCode, NodeBundle, Parent, Query,
    ReceivedCharacter, Rect, Res, Size, TextBundle, Visibility, With,
};
use bevy::text::{Font, HorizontalAlign, Text, TextAlignment, TextStyle, VerticalAlign};
use bevy::ui::{Interaction, Style, UiColor};
use clipboard::{ClipboardContext, ClipboardProvider};
use glyph_brush::{GlyphCalculatorBuilder, GlyphCruncher, Section};

use crate::{Commands, MouseButton, PositionType, Val};

/// A list of [`TextInputConstrain`]s. The character won't be added to the
/// input if any of these returns false
#[derive(Component)]
pub struct TextInputConstrains(pub Vec<Box<dyn TextInputConstrain + Send + Sync + 'static>>);

impl TextInputConstrains {
    pub fn test(&self, old: &str, new: &str) -> bool {
        self.0.iter().all(|constrain| constrain.test(old, new))
    }
}

pub trait TextInputConstrain {
    /// Returns true if the character(s) can be appended/inserted to the input field
    fn test(&self, old: &str, new: &str) -> bool;
}

/// Default text input constrains
#[derive(Clone, Debug)]
pub enum DefaultConstrains {
    /// Only allow these characters
    AllowedCharacters(Vec<char>),
    /// Disallow these characters
    DisallowedCharacters(Vec<char>),
    /// Max input length
    MaxLength(usize),
}

impl TextInputConstrain for DefaultConstrains {
    fn test(&self, _old: &str, new: &str) -> bool {
        match self {
            DefaultConstrains::AllowedCharacters(chars) => {
                new.chars().all(|ch| chars.contains(&ch))
            }
            DefaultConstrains::DisallowedCharacters(chars) => {
                !new.chars().any(|ch| chars.contains(&ch))
            }
            DefaultConstrains::MaxLength(len) => new.len() <= *len,
        }
    }
}

/// Text that will be displayed when the input is empty
#[derive(Default, Component, Clone, Debug)]
pub struct PlaceholderText(pub Text);
/// Style of the input text
#[derive(Default, Component, Clone, Debug)]
pub struct InputTextStyle(pub TextStyle);
/// Alignment of the input text
#[derive(Default, Component, Clone, Debug)]
pub struct InputTextAlignment(pub TextAlignment);

impl From<Text> for PlaceholderText {
    fn from(inner: Text) -> Self {
        Self(inner)
    }
}

impl From<TextStyle> for InputTextStyle {
    fn from(inner: TextStyle) -> Self {
        Self(inner)
    }
}

impl From<TextAlignment> for InputTextAlignment {
    fn from(inner: TextAlignment) -> Self {
        Self(inner)
    }
}

/// If the text input is focused, it will hold cursor index
#[derive(Component, Default, Debug, Clone)]
pub struct TextInputFocus(pub Option<usize>);

/// A blinking thing that appears when you focus on a text input.
/// A bundle that will be spawned with [`TextCursor`] component.
/// Added as a component to `TextInputBundle`
#[derive(Component, Clone, Debug)]
pub struct TextCursorStyle(pub NodeBundle);

impl TextCursorStyle {
    /// The default blinking cursor. Params should with input's params
    pub fn default(
        font_size: f32,
        color: UiColor,
        padding: Rect<Val>,
        alignment: TextAlignment,
    ) -> Self {
        Self(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(font_size / 12.0), Val::Px(font_size)),
                margin: Rect {
                    left: if alignment.horizontal == HorizontalAlign::Left {
                        padding.left
                    } else {
                        Val::Undefined
                    },
                    right: if alignment.horizontal == HorizontalAlign::Right {
                        padding.right
                    } else {
                        Val::Undefined
                    },
                    top: if alignment.vertical == VerticalAlign::Top {
                        padding.top
                    } else {
                        Val::Undefined
                    },
                    bottom: if alignment.vertical == VerticalAlign::Bottom {
                        padding.bottom
                    } else {
                        Val::Undefined
                    },
                },
                ..Default::default()
            },
            color,
            ..Default::default()
        })
    }
}

#[derive(Component, Clone, Default, Debug)]
pub struct TextCursor;

#[derive(Component, Clone, Default, Debug)]
pub struct TextInputValue(pub String);

impl Deref for TextInputValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn text_input_unfocus_system(
    input: Res<Input<MouseButton>>,
    mut text_inputs: Query<&mut TextInputFocus>,
) {
    if input.just_pressed(MouseButton::Left) {
        for mut focus in text_inputs.iter_mut() {
            focus.0 = None;
        }
    }
}

pub fn text_input_focus_on_click_system(
    mut query: Query<(&mut TextInputFocus, &Interaction, &TextInputValue), Changed<Interaction>>,
) {
    for (mut focus, interaction, value) in query.iter_mut() {
        if *interaction == Interaction::Clicked {
            focus.0 = Some(value.len());
        }
    }
}

pub fn text_input_move_cursor_system(
    mut commands: Commands,
    fonts: Res<Assets<Font>>,
    query: Query<
        (
            Entity,
            &TextInputFocus,
            &InputTextStyle,
            &TextInputValue,
            &TextCursorStyle,
            &CursorBlinkingInterval,
        ),
        Changed<TextInputFocus>,
    >,
    mut query_cursors: Query<(Entity, &mut Style, &Parent), With<TextCursor>>,
) {
    'text: for (entity, focus, text_style, value, cursor_style, cursor_interval) in query.iter() {
        if let Some(char_index) = focus.0 {
            for (_, mut style, parent) in query_cursors.iter_mut() {
                if parent.0 == entity {
                    let font = fonts.get(text_style.0.font.clone()).unwrap().font.clone();

                    let text_before_cursor = &value.0[..char_index];
                    let font_size = text_style.0.font_size;
                    let scale = PxScale {
                        x: font_size,
                        y: font_size,
                    };
                    let x = GlyphCalculatorBuilder::using_font(font.clone())
                        .build()
                        .cache_scope()
                        .glyph_bounds(
                            Section::default().add_text(
                                glyph_brush::Text::new(
                                    text_before_cursor.split('\n').last().unwrap(),
                                )
                                .with_scale(scale),
                            ),
                        )
                        .map(|rect| rect.width())
                        .unwrap_or_default();
                    let lines_before_cursor = text_before_cursor.split('\n').count();
                    let lines_total = value.0.split('\n').count();
                    let y =
                        font.as_scaled(scale).height() * (lines_total - lines_before_cursor) as f32;
                    style.position.left = Val::Px(x);
                    style.position.top = Val::Px(-y);

                    let current_glyph_bounds = font.glyph_bounds(&Glyph {
                        id: font.glyph_id(value.0.chars().nth(char_index).unwrap_or(' ')),
                        scale,
                        position: Default::default(),
                    });
                    if cursor_style.0.style.size.width == Val::Auto {
                        style.size.width = Val::Px(current_glyph_bounds.width());
                    }
                    if cursor_style.0.style.size.height == Val::Auto {
                        style.size.height = Val::Px(current_glyph_bounds.height());
                    }
                    continue 'text;
                }
            }
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn_bundle(cursor_style.0.clone())
                    .insert(TextCursor)
                    .insert(BlinkingTimer(Timer::new(cursor_interval.0, true)));
            });
        } else {
            for (cursor, _, parent) in query_cursors.iter_mut() {
                if parent.0 == entity {
                    commands.entity(cursor).despawn_recursive();
                }
            }
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct CursorBlinkingInterval(pub Duration);

impl Default for CursorBlinkingInterval {
    fn default() -> Self {
        Self(Duration::from_millis(750))
    }
}

#[derive(Component)]
pub struct BlinkingTimer(pub Timer);

pub fn text_input_blink_cursor_system(
    time: Res<Time>,
    mut query: Query<(&mut Visibility, &mut BlinkingTimer), With<TextCursor>>,
) {
    for (mut visibility, mut timer) in query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

#[derive(Component)]
pub struct TextInputPlaceholder;
#[derive(Component)]
pub struct TextInputInner;

pub fn text_input_create_system(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &PlaceholderText,
            &InputTextStyle,
            &InputTextAlignment,
            &TextInputValue,
        ),
        Added<PlaceholderText>,
    >,
) {
    for (entity, placeholder, style, alignment, value) in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    text: placeholder.0.clone(),
                    visibility: Visibility {
                        is_visible: value.is_empty(),
                    },
                    ..Default::default()
                })
                .insert(TextInputPlaceholder);

            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    text: Text::with_section(&value.0, style.0.clone(), alignment.0),
                    ..Default::default()
                })
                .insert(TextInputInner);
        });
    }
}

pub fn text_input_update_system(
    query: Query<(Entity, &TextInputValue), Changed<TextInputValue>>,
    mut placeholder_query: Query<(&Parent, &mut Visibility), With<TextInputPlaceholder>>,
    mut value_query: Query<(&Parent, &mut Text), With<TextInputInner>>,
) {
    for (entity, value) in query.iter() {
        let mut placeholder_visibility = placeholder_query
            .iter_mut()
            .find(|(parent, _)| parent.0 == entity)
            .unwrap()
            .1;
        let mut inner_text = value_query
            .iter_mut()
            .find(|(parent, _)| parent.0 == entity)
            .unwrap()
            .1;
        placeholder_visibility.is_visible = value.is_empty();
        inner_text.sections[0].value = value.0.clone();
        if inner_text.sections[0].value.ends_with('\n') {
            inner_text.sections[0].value.push(' ');
        }
    }
}

pub fn text_input_system(
    fonts: Res<Assets<Font>>,
    mut query: Query<(
        Entity,
        &InputTextStyle,
        &mut TextInputValue,
        &mut TextInputFocus,
        &TextInputConstrains,
    )>,
    mut cursors: Query<(&Parent, &mut Visibility, &mut BlinkingTimer)>,
    mut input: EventReader<KeyboardInput>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    let keys = input
        .iter()
        .filter(|key| key.state == ElementState::Pressed)
        .filter_map(|key| key.key_code)
        .collect::<Vec<_>>();
    let chars_all = char_evr.iter().map(|rc| rc.char).collect::<Vec<_>>();
    let s = chars_all
        .iter()
        .copied()
        .filter(|ch| !ch.is_control())
        .collect::<String>();
    let control_chars = chars_all
        .iter()
        .copied()
        .filter(|ch| ch.is_control())
        .collect::<Vec<_>>();
    for (entity, style, mut value, mut focus, constrains) in query.iter_mut() {
        if let Some(cursor) = focus.0.as_mut() {
            let font = fonts.get(style.0.font.clone()).unwrap().font.clone();
            let mut new_value = value.0.clone();
            let mut new_cursor = *cursor;
            if control_chars.contains(&'\r') {
                // new line
                new_value.insert(new_cursor, '\n');
                new_cursor += 1;
            }
            if control_chars.contains(&'\u{1}') {
                // Ctrl-A
                new_cursor = 0;
            }
            if control_chars.contains(&'\u{5}') {
                // Ctrl-E
                new_cursor = new_value.len();
            }
            if control_chars.contains(&'\u{8}') && new_cursor != 0 {
                // backspace
                new_value.remove(new_cursor - 1);
                new_cursor -= 1;
            }
            if control_chars.contains(&'\u{7f}') && new_cursor < new_value.len() {
                // delete
                new_value.remove(new_cursor);
            }
            if control_chars.contains(&'\u{16}') {
                // paste
                if let Ok(mut clipboard) = ClipboardContext::new() {
                    if let Ok(contents) = clipboard.get_contents() {
                        new_value.insert_str(new_cursor, &contents);
                        new_cursor += contents.len();
                    }
                }
            }
            if keys.contains(&KeyCode::Left) && new_cursor > 0 {
                new_cursor -= 1;
            }
            if keys.contains(&KeyCode::Right) && new_cursor < new_value.len() {
                new_cursor += 1;
            }

            if keys.contains(&KeyCode::Home) {
                new_cursor -= new_value[..new_cursor]
                    .chars()
                    .rev()
                    .position(|ch| ch == '\n')
                    .unwrap_or(new_cursor);
            }
            if keys.contains(&KeyCode::End) {
                new_cursor += new_value[new_cursor..]
                    .chars()
                    .position(|ch| ch == '\n')
                    .unwrap_or(new_value.len() - new_cursor);
            }

            let scale = PxScale {
                x: style.0.font_size,
                y: style.0.font_size,
            };
            if keys.contains(&KeyCode::Up) {
                if new_value[..new_cursor].split('\n').count() <= 1 {
                    new_cursor = 0;
                } else {
                    let mut lines_before_cursor = new_value[..new_cursor].split('\n').rev();
                    let current_line_before_cursor = lines_before_cursor.next().unwrap();
                    let previous_line = lines_before_cursor.next().unwrap();
                    let target_width = text_width(current_line_before_cursor, font.clone(), scale);
                    let x = (0..=previous_line.len())
                        .map(|i| (i, text_width(&previous_line[..i], font.clone(), scale)))
                        .min_by(|(_, width1), (_, width2)| {
                            // width is never f32::NAN, so unwrap is safe
                            (width1 - target_width)
                                .abs()
                                .partial_cmp(&(width2 - target_width).abs())
                                .unwrap()
                        })
                        .unwrap()
                        .0;
                    new_cursor = x + lines_before_cursor
                        .map(|line| line.len() + 1)
                        .sum::<usize>();
                }
            }
            if keys.contains(&KeyCode::Down) {
                if new_value[new_cursor..].split('\n').count() <= 1 {
                    new_cursor = new_value.len();
                } else {
                    let mut lines_after_cursor = new_value[new_cursor..].split('\n');
                    let current_line_before_cursor =
                        new_value[..new_cursor].split('\n').last().unwrap();
                    let current_line_after_cursor = lines_after_cursor.next().unwrap();
                    let next_line = lines_after_cursor.next().unwrap();
                    let target_width = text_width(current_line_before_cursor, font.clone(), scale);
                    let x = (0..=next_line.len())
                        .map(|i| (i, text_width(&next_line[..i], font.clone(), scale)))
                        .min_by(|(_, width1), (_, width2)| {
                            // width is never f32::NAN, so unwrap is safe
                            (width1 - target_width)
                                .abs()
                                .partial_cmp(&(width2 - target_width).abs())
                                .unwrap()
                        })
                        .unwrap()
                        .0;
                    new_cursor = new_cursor + current_line_after_cursor.len() + 1 + x;
                }
            }

            new_value.insert_str(new_cursor, &s);
            new_cursor += s.len();

            if value.0 != new_value || *cursor != new_cursor {
                if value.0 != new_value && !constrains.test(&value.0, &new_value) {
                    continue;
                }
                value.0 = new_value;
                *cursor = new_cursor;
                for (parent, mut visibility, mut timer) in cursors.iter_mut() {
                    if parent.0 == entity {
                        visibility.is_visible = true;
                        timer.0.reset();
                        break;
                    }
                }
            }
        }
    }
}

fn text_width(text: &str, font: FontArc, scale: PxScale) -> f32 {
    GlyphCalculatorBuilder::using_font(font)
        .build()
        .cache_scope()
        .glyph_bounds(Section::default().add_text(glyph_brush::Text::new(text).with_scale(scale)))
        .map(|rect| rect.width())
        .unwrap_or_default()
}
