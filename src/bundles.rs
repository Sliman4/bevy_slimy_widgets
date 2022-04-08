use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::progress_bar::Progress;
use crate::text_input::{
    CursorBlinkingInterval, DefaultConstrains, InputTextAlignment, InputTextStyle, PlaceholderText,
    TextCursorStyle, TextInputConstrains, TextInputFocus, TextInputTargetSize, TextInputValue,
};

/// A UI node that is a progress bar
///
/// In order to work as expected, you must add the animation marker component
/// to define how to display progress. Use built-in [`ProgressBarSizeAnimation`](crate::progress_bar::ProgressBarSizeAnimation)
/// or implement your own animation:
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_slimy_widgets::ProgressBarBundle;
/// # use bevy_slimy_widgets::progress_bar::Progress;
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
///             hue: **progress,
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
            progress: Progress::empty(),
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

/// A text input field. It has a child with [`TextCursor`] component
/// when focused, and is focused on click.
#[derive(Bundle)]
pub struct TextInputBundle {
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Describes whether and how the text field has been interacted with by the input
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
    /// Text that will be displayed when the input is empty
    pub placeholder: PlaceholderText,
    /// Style of the input text
    pub text_style: InputTextStyle,
    /// Alignment of the input text
    pub text_alignment: InputTextAlignment,
    /// The character won't be added to the input if any of these returns false
    pub constrains: TextInputConstrains,
    /// Whether the text input is focused or not.
    /// If the text input is focused, it will hold cursor index
    pub focus: TextInputFocus,
    /// A blinking thing that appears when you focus on a text input.
    /// A bundle that will be spawned with [`TextCursor`] component
    pub cursor: TextCursorStyle,
    /// Text field's value, text that is typed in here
    pub value: TextInputValue,
    /// Text cursor blinking interval. Default is 750ms
    pub cursor_blinking_interval: CursorBlinkingInterval,
    /// If present, it will decrease font size to fit into target size
    pub target_size: TextInputTargetSize,
}

impl Default for TextInputBundle {
    fn default() -> Self {
        Self {
            node: Default::default(),
            style: Default::default(),
            interaction: Default::default(),
            focus_policy: Default::default(),
            color: Default::default(),
            image: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            placeholder: Default::default(),
            text_style: Default::default(),
            text_alignment: Default::default(),
            constrains: TextInputConstrains(vec![Box::new(
                DefaultConstrains::DisallowedCharacters(vec!['\n']),
            )]),
            focus: Default::default(),
            cursor: TextCursorStyle::default(
                TextStyle::default().font_size,
                TextStyle::default().color.into(),
                Default::default(),
                Default::default(),
            ),
            value: Default::default(),
            cursor_blinking_interval: Default::default(),
            target_size: Default::default(),
        }
    }
}
