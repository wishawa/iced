//! Display a horizontal or vertical rule for dividing content.

use std::hash::Hash;

use crate::layout;
use crate::renderer::{self, Renderer};
use crate::{Element, Hasher, Layout, Length, Point, Rectangle, Size, Widget};

pub use iced_style::rule::{Style, StyleSheet};

/// Display a horizontal or vertical rule for dividing content.
#[derive(Debug, Copy, Clone)]
pub struct Rule<'a> {
    width: Length,
    height: Length,
    style: &'a dyn StyleSheet,
    is_horizontal: bool,
}

impl<'a> Rule<'a> {
    /// Creates a horizontal [`Rule`] for dividing content by the given vertical spacing.
    pub fn horizontal(spacing: u16) -> Self {
        Rule {
            width: Length::Fill,
            height: Length::from(Length::Units(spacing)),
            style: Renderer::Style::default(),
            is_horizontal: true,
        }
    }

    /// Creates a vertical [`Rule`] for dividing content by the given horizontal spacing.
    pub fn vertical(spacing: u16) -> Self {
        Rule {
            width: Length::from(Length::Units(spacing)),
            height: Length::Fill,
            style: Renderer::Style::default(),
            is_horizontal: false,
        }
    }

    /// Sets the style of the [`Rule`].
    pub fn style<'b>(mut self, style: impl Into<&'b dyn StyleSheet>) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Rule<'a> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        layout::Node::new(limits.resolve(Size::ZERO))
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        _defaults: &renderer::Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        renderer.draw(layout.bounds(), &self.style, self.is_horizontal)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
    }
}

impl<'a, Message> From<Rule<'a>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(rule: Rule<'a>) -> Element<'a, Message> {
        Element::new(rule)
    }
}
