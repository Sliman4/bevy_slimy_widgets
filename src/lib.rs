#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![deny(broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

use crate::widgets::progress_bar::progress_bar_size_animation_system;
use bevy::prelude::*;

mod bundles;
mod widgets;

pub use bundles::*;
pub use widgets::*;

/// A plugin struct. Use this with [`App::add_plugin()`]
pub struct SlimyWidgetsPlugin;

impl Plugin for SlimyWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            progress_bar_size_animation_system.label(SystemLabels::ProgressBarSizeAnimation),
        );
    }
}

/// [`Labels`](bevy::ecs::schedule::SystemLabel) in [`bevy`] are used for system ordering.
/// See [System Order of Execution][cheatbook_system_order] on unofficial bevy cheatbook for details.
///
/// [cheatbook_system_order]: https://bevy-cheatbook.github.io/programming/system-order.html
#[derive(SystemLabel, Clone, Hash, PartialEq, Eq, Debug)]
pub enum SystemLabels {
    /// [`ProgressBarBundle`]'s [`ProgressBarSizeAnimation`] animation system
    ProgressBarSizeAnimation,
}
