#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub use bundles::*;
pub use widgets::*;

use crate::text_input::{
    text_input_blink_cursor_system, text_input_create_system, text_input_focus_on_click_system,
    text_input_move_cursor_system, text_input_system, text_input_unfocus_system,
    text_input_update_system,
};
use crate::widgets::progress_bar::progress_bar_size_animation_system;

mod bundles;
mod widgets;

/// A plugin struct. Use this with [`App::add_plugin()`]
pub struct SlimyWidgetsPlugin;

impl Plugin for SlimyWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            progress_bar_size_animation_system.label(SystemLabels::ProgressBarSizeAnimation),
        )
        .add_system(text_input_unfocus_system.before(SystemLabels::TextInputFocusOnClick))
        .add_system(text_input_focus_on_click_system.label(SystemLabels::TextInputFocusOnClick))
        .add_system(text_input_move_cursor_system.label(SystemLabels::TextInputMoveCursor))
        .add_system(text_input_blink_cursor_system.label(SystemLabels::TextInputBlinkCursor))
        .add_system(text_input_create_system)
        .add_system_to_stage(CoreStage::PostUpdate, text_input_update_system)
        .add_system(text_input_system.before(SystemLabels::TextInputBlinkCursor));
    }
}

/// [`Labels`](bevy::ecs::schedule::SystemLabel) in [`bevy`] are used for system ordering.
/// See [System Order of Execution][cheatbook_system_order] on unofficial bevy cheatbook for details.
///
/// [cheatbook_system_order]: https://bevy-cheatbook.github.io/programming/system-order.html
#[derive(SystemLabel, Clone, Hash, PartialEq, Eq, Debug)]
pub enum SystemLabels {
    /// [`ProgressBarBundle`]'s [`ProgressBarSizeAnimation`](crate::progress_bar::ProgressBarSizeAnimation) animation system
    ProgressBarSizeAnimation,
    /// Focus [`TextInputBundle`] when clicked on it
    TextInputFocusOnClick,
    /// Move [`TextInputBundle`]'s cursor
    TextInputMoveCursor,
    /// [`TextInputBundle`]'s cursor blinking
    TextInputBlinkCursor,
}
