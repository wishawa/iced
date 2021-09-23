//! Show toggle controls using togglers.
use std::hash::Hash;

use crate::alignment;
use crate::event;
use crate::layout;
use crate::mouse;
use crate::renderer::{self, Renderer};
use crate::{
    Alignment, Clipboard, Element, Event, Font, Hasher, Layout, Length, Point,
    Rectangle, Row, Text, Widget,
};

pub use iced_style::toggler::{Style, StyleSheet};

/// A toggler widget
///
/// # Example
///
/// ```
/// # type Toggler<Message> = iced_native::Toggler<Message, iced_native::renderer::Null>;
/// #
/// pub enum Message {
///     TogglerToggled(bool),
/// }
///
/// let is_active = true;
///
/// Toggler::new(is_active, String::from("Toggle me!"), |b| Message::TogglerToggled(b));
/// ```
#[allow(missing_debug_implementations)]
pub struct Toggler<'a, Message> {
    is_active: bool,
    on_toggle: Box<dyn Fn(bool) -> Message>,
    label: Option<String>,
    width: Length,
    size: Option<u16>,
    text_size: Option<u16>,
    text_alignment: alignment::Horizontal,
    spacing: u16,
    font: Font,
    style: &'a dyn StyleSheet,
}

impl<'a, Message> Toggler<'a, Message> {
    /// Creates a new [`Toggler`].
    ///
    /// It expects:
    ///   * a boolean describing whether the [`Toggler`] is checked or not
    ///   * An optional label for the [`Toggler`]
    ///   * a function that will be called when the [`Toggler`] is toggled. It
    ///     will receive the new state of the [`Toggler`] and must produce a
    ///     `Message`.
    pub fn new<F>(
        is_active: bool,
        label: impl Into<Option<String>>,
        f: F,
    ) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        Toggler {
            is_active,
            on_toggle: Box::new(f),
            label: label.into(),
            width: Length::Fill,
            size: None,
            text_size: None,
            text_alignment: alignment::Horizontal::Left,
            spacing: 0,
            font: Font::default(),
            style: Default::default(),
        }
    }

    /// Sets the size of the [`Toggler`].
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the width of the [`Toggler`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the text size o the [`Toggler`].
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the horizontal alignment of the text of the [`Toggler`]
    pub fn text_alignment(mut self, alignment: alignment::Horizontal) -> Self {
        self.text_alignment = alignment;
        self
    }

    /// Sets the spacing between the [`Toggler`] and the text.
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the [`Font`] of the text of the [`Toggler`]
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`Toggler`].
    pub fn style<'b>(mut self, style: impl Into<&'b dyn StyleSheet>) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Toggler<'a, Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let mut row = Row::<()>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Alignment::Center);

        if let Some(label) = &self.label {
            row = row.push(
                Text::new(label)
                    .horizontal_alignment(self.text_alignment)
                    .font(self.font)
                    .width(self.width)
                    .size(self.text_size.unwrap_or(renderer.default_size())),
            );
        }

        row = row.push(
            Row::new()
                .width(Length::Units(2 * self.size))
                .height(Length::Units(self.size)),
        );

        row.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        _renderer: &dyn Renderer,
        _clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let mouse_over = layout.bounds().contains(cursor_position);

                if mouse_over {
                    messages.push((self.on_toggle)(!self.is_active));

                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            _ => event::Status::Ignored,
        }
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        defaults: &renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let mut children = layout.children();

        let label = match &self.label {
            Some(label) => {
                let label_layout = children.next().unwrap();

                Some(renderer.fill_text(
                    defaults,
                    label_layout.bounds(),
                    &label,
                    self.text_size.unwrap_or(renderer.default_size()),
                    self.font,
                    None,
                    self.text_alignment,
                    alignment::Vertical::Center,
                ))
            }

            None => None,
        };

        let toggler_layout = children.next().unwrap();
        let toggler_bounds = toggler_layout.bounds();

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            toggler_bounds,
            self.is_active,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.label.hash(state)
    }
}

impl<'a, Message> From<Toggler<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(toggler: Toggler<'a, Message>) -> Element<'a, Message> {
        Element::new(toggler)
    }
}
