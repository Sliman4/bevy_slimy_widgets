use crate::Progress;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

/// A UI node that is a progress bar
///
/// In order to work as expected, you must add the animation marker component
/// to define how to display progress. Use built-in [`ProgressBarSizeAnimation`](crate::ProgressBarSizeAnimation)
/// or implement your own animation:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_slimy_widgets::{Progress, ProgressBarBundle};
///
/// #[derive(Component)]
/// struct MyProgressBarAnimation;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn_bundle(UiCameraBundle::default());
///
///     commands.spawn_bundle(ProgressBarBundle {
///         style: Style {
///             size: Size::new(Val::Px(100.0), Val::Px(100.0)),
///             ..Default::default()
///         },
///         ..Default::default()
///     }).insert(MyProgressBarAnimation);
/// }
///
/// fn animation_system(mut query: Query<(&Progress, &mut UiColor), (With<MyProgressBarAnimation>, Changed<Progress>)>) {
///     for (progress, mut color) in query.iter_mut() {
///         // change hue from 0 to 100 (from red to green)
///         color.0 = Color::Hsla {
///             hue: *progress,
///             saturation: 0.7,
///             lightness: 0.5,
///             alpha: 1.0,
///         };
///     }
/// }
/// ```
#[derive(Bundle, Clone, Debug)]
pub struct ProgressBarBundle {
    /// Describes the size of the node
    pub node: Node,
    /// Describes the progress of the bar
    pub progress: Progress,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes whether and how the button has been interacted with by the input
    pub interaction: Interaction,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The color of the node
    pub color: UiColor,
    /// The image of the node
    pub image: UiImage,
    /// The transform of the node
    pub transform: Transform,
    /// The global transform of the node
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
}

impl Default for ProgressBarBundle {
    fn default() -> Self {
        ProgressBarBundle {
            progress: Progress::default(),
            interaction: Default::default(),
            focus_policy: Default::default(),
            node: Default::default(),
            style: Default::default(),
            color: Default::default(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
        }
    }
}
