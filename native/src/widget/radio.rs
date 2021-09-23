//! Create choices using radio buttons.
use std::hash::Hash;

use crate::alignment::{self, Alignment};
use crate::event::{self, Event};
use crate::layout;
use crate::mouse;
use crate::renderer::{self, Renderer};
use crate::touch;
use crate::{
    Clipboard, Color, Element, Font, Hasher, Layout, Length, Point, Rectangle,
    Row, Text, Widget,
};

pub use iced_style::radio::{Style, StyleSheet};

/// A circular button representing a choice.
///
/// # Example
/// ```
/// # type Radio<Message> =
/// #     iced_native::Radio<Message, iced_native::renderer::Null>;
/// #
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// pub enum Choice {
///     A,
///     B,
/// }
///
/// #[derive(Debug, Clone, Copy)]
/// pub enum Message {
///     RadioSelected(Choice),
/// }
///
/// let selected_choice = Some(Choice::A);
///
/// Radio::new(Choice::A, "This is A", selected_choice, Message::RadioSelected);
///
/// Radio::new(Choice::B, "This is B", selected_choice, Message::RadioSelected);
/// ```
///
/// ![Radio buttons drawn by `iced_wgpu`](https://github.com/hecrj/iced/blob/7760618fb112074bc40b148944521f312152012a/docs/images/radio.png?raw=true)
#[allow(missing_debug_implementations)]
pub struct Radio<'a, Message> {
    is_selected: bool,
    on_click: Message,
    label: String,
    width: Length,
    size: u16,
    spacing: u16,
    text_size: Option<u16>,
    text_color: Option<Color>,
    font: Font,
    style: &'a dyn StyleSheet,
}

impl<'a, Message> Radio<'a, Message>
where
    Message: Clone,
{
    /// Creates a new [`Radio`] button.
    ///
    /// It expects:
    ///   * the value related to the [`Radio`] button
    ///   * the label of the [`Radio`] button
    ///   * the current selected value
    ///   * a function that will be called when the [`Radio`] is selected. It
    ///   receives the value of the radio and must produce a `Message`.
    pub fn new<F, V>(
        value: V,
        label: impl Into<String>,
        selected: Option<V>,
        f: F,
    ) -> Self
    where
        V: Eq + Copy,
        F: 'static + Fn(V) -> Message,
    {
        Radio {
            is_selected: Some(value) == selected,
            on_click: f(value),
            label: label.into(),
            width: Length::Shrink,
            size: 28,
            spacing: 15,
            text_size: None,
            text_color: None,
            font: Default::default(),
            style: Renderer::Style::default(),
        }
    }

    /// Sets the size of the [`Radio`] button.
    pub fn size(mut self, size: u16) -> Self {
        self.size = size;
        self
    }

    /// Sets the width of the [`Radio`] button.
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the spacing between the [`Radio`] button and the text.
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the text size of the [`Radio`] button.
    pub fn text_size(mut self, text_size: u16) -> Self {
        self.text_size = Some(text_size);
        self
    }

    /// Sets the text color of the [`Radio`] button.
    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    /// Sets the text font of the [`Radio`] button.
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    /// Sets the style of the [`Radio`] button.
    pub fn style<'b>(mut self, style: impl Into<&'b dyn StyleSheet>) -> Self
    where
        'b: 'a,
    {
        self.style = style.into();
        self
    }
}

impl<'a, Message> Widget<Message> for Radio<'a, Message>
where
    Message: Clone,
{
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
        Row::<()>::new()
            .width(self.width)
            .spacing(self.spacing)
            .align_items(Alignment::Center)
            .push(
                Row::new()
                    .width(Length::Units(self.size))
                    .height(Length::Units(self.size)),
            )
            .push(
                Text::new(&self.label)
                    .width(self.width)
                    .size(self.text_size.unwrap_or(renderer.default_size())),
            )
            .layout(renderer, limits)
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
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if layout.bounds().contains(cursor_position) {
                    messages.push(self.on_click.clone());

                    return event::Status::Captured;
                }
            }
            _ => {}
        }

        event::Status::Ignored
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

        let radio_layout = children.next().unwrap();
        let label_layout = children.next().unwrap();
        let radio_bounds = radio_layout.bounds();

        let label = renderer.fill_text(
            renderer,
            defaults,
            label_layout.bounds(),
            &self.label,
            self.text_size.unwrap_or(renderer.default_size()),
            self.font,
            self.text_color,
            alignment::Horizontal::Left,
            alignment::Vertical::Center,
        );

        let is_mouse_over = bounds.contains(cursor_position);

        self::Renderer::draw(
            renderer,
            radio_bounds,
            self.is_selected,
            is_mouse_over,
            label,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.label.hash(state);
    }
}

impl<'a, Message> From<Radio<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(radio: Radio<'a, Message>) -> Element<'a, Message> {
        Element::new(radio)
    }
}
