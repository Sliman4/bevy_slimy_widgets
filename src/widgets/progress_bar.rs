//! A progress bar widget. You may want to use this with [`bevy_loading`](https://github.com/IyesGames/bevy_loading).

use bevy::prelude::*;
use std::ops::{AddAssign, Deref};

/// Progress struct for ProgressBar.
/// ```
/// # use bevy::prelude::Progress;
/// # use bevy_ui::Val;
///
/// let mut progress_bar = Progress::new(25.0);
///
/// *progress_bar += 50.0;
///
/// let progress_bar_width = Val::Percent(*progress_bar);
/// ```
///
/// Note: values will be clamped between 0.0 and 100.0
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Progress(f32);

impl Deref for Progress {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Progress {
    /// Creates a new instance of [`Progress`]
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// // 100%
    /// let progress_bar = Progress::new(100.0);
    /// ```
    pub fn new(value: f32) -> Self {
        Progress(Self::clamp_value(value))
    }

    /// Creates a new instance of [`Progress`] with 0% done
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// let empty_progress_bar = Progress::empty();
    /// assert!(*empty_progress_bar, 0.0);
    /// ```
    pub fn empty() -> Self {
        Self::new(0.0)
    }

    /// Sets the progress value
    pub fn set(&mut self, value: f32) {
        self.0 = Self::clamp_value(value)
    }

    /// Check if this [`Progress`] has reached 100%
    /// ```
    /// # use bevy::prelude::Progress;
    ///
    /// let mut progress_bar = Progress::new(50.0);
    /// assert!(!empty_progress_bar.is_done());
    ///
    /// *progress_bar += 50.0;
    /// assert!(empty_progress_bar.is_done());
    /// ```
    pub fn is_done(&self) -> bool {
        (self.0 - 100.0).abs() < f32::EPSILON
    }

    fn clamp_value(value: f32) -> f32 {
        value.clamp(0.0, 100.0)
    }
}

impl AddAssign<f32> for Progress {
    fn add_assign(&mut self, rhs: f32) {
        self.set(**self + rhs)
    }
}

/// Progress bar resize animation
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ProgressBarSizeAnimation {
    /// The width of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    Width,
    /// The height of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    Height,
    /// Both the width and the height of a node will be changed to [`Val::Percent`]\(*progress)
    /// when the [`Progress`] changes
    Both,
}

impl Default for ProgressBarSizeAnimation {
    fn default() -> Self {
        ProgressBarSizeAnimation::Width
    }
}

/// Updates progress bar [`Size`] if [`Progress`] has changed
pub fn progress_bar_size_animation_system(
    mut query: Query<(&Progress, &ProgressBarSizeAnimation, &mut Style), Changed<Progress>>,
) {
    for (progress, dimension, mut style) in query.iter_mut() {
        let (resize_width, resize_height) = match dimension {
            ProgressBarSizeAnimation::Width => (true, false),
            ProgressBarSizeAnimation::Height => (false, true),
            ProgressBarSizeAnimation::Both => (true, true),
        };
        if resize_width {
            style.size.width = Val::Percent(**progress);
        }
        if resize_height {
            style.size.height = Val::Percent(**progress);
        }
    }
}
