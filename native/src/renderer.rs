//! Write your own renderer.
//!
//! You will need to implement the `Renderer` trait first. It simply contains
//! an `Output` associated type.
//!
//! There is no common trait to draw all the widgets. Instead, every [`Widget`]
//! constrains its generic `Renderer` type as necessary.
//!
//! This approach is flexible and composable. For instance, the
//! [`Text`] widget only needs a [`text::Renderer`] while a [`Checkbox`] widget
//! needs both a [`text::Renderer`] and a [`checkbox::Renderer`], reusing logic.
//!
//! In the end, a __renderer__ satisfying all the constraints is
//! needed to build a [`UserInterface`].
//!
//! [`Widget`]: crate::Widget
//! [`UserInterface`]: crate::UserInterface
//! [`Text`]: crate::widget::Text
//! [`text::Renderer`]: crate::widget::text::Renderer
//! [`Checkbox`]: crate::widget::Checkbox
//! [`checkbox::Renderer`]: crate::widget::checkbox::Renderer

use crate::{Color, Rectangle};

/// A component that can take the state of a user interface and produce an
/// output for its users.
pub trait Renderer {
    /// After layout call back.
    ///
    /// You should override this if you need to perform any operations after
    /// layouting. For instance, trimming the measurements cache.
    fn after_layout(&mut self) {}

    fn begin_layer(&mut self, bounds: Rectangle);
    fn end_layer(&mut self);
}

/// Some default styling attributes.
#[derive(Debug, Clone, Copy)]
pub struct Defaults {
    /// Text styling
    pub text: Text,
}

impl Default for Defaults {
    fn default() -> Defaults {
        Defaults {
            text: Text::default(),
        }
    }
}

/// Some default text styling attributes.
#[derive(Debug, Clone, Copy)]
pub struct Text {
    /// The default color of text
    pub color: Color,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            color: Color::BLACK,
        }
    }
}
