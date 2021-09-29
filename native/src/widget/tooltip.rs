//! Display a widget over another.
use std::hash::Hash;

use iced_core::Rectangle;

use crate::event;
use crate::layout;
use crate::renderer::{self, Renderer};
use crate::widget::container;
use crate::widget::text::Text;
use crate::{
    Clipboard, Element, Event, Font, Hasher, Layout, Length, Padding, Point,
    Size, Vector, Widget,
};

/// An element to display a widget over another.
#[allow(missing_debug_implementations)]
pub struct Tooltip<'a, Message> {
    content: Element<'a, Message>,
    tooltip: Text,
    position: Position,
    style_sheet: &'a dyn container::StyleSheet,
    gap: u16,
    padding: u16,
}

impl<'a, Message> Tooltip<'a, Message> {
    pub const DEFAULT_PADDING: u16 = 5;

    /// Creates an empty [`Tooltip`].
    ///
    /// [`Tooltip`]: struct.Tooltip.html
    pub fn new(
        content: impl Into<Element<'a, Message>>,
        tooltip: impl ToString,
        position: Position,
    ) -> Self {
        Tooltip {
            content: content.into(),
            tooltip: Text::new(tooltip.to_string()),
            position,
            style_sheet: Default::default(),
            gap: 0,
            padding: Self::DEFAULT_PADDING,
        }
    }

    /// Sets the size of the text of the [`Tooltip`].
    pub fn size(mut self, size: u16) -> Self {
        self.tooltip = self.tooltip.size(size);
        self
    }

    /// Sets the font of the [`Tooltip`].
    ///
    /// [`Font`]: Renderer::Font
    pub fn font(mut self, font: impl Into<Font>) -> Self {
        self.tooltip = self.tooltip.font(font);
        self
    }

    /// Sets the gap between the content and its [`Tooltip`].
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the padding of the [`Tooltip`].
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the style of the [`Tooltip`].
    pub fn style<'b>(
        mut self,
        style_sheet: &'b dyn container::StyleSheet,
    ) -> Self
    where
        'b: 'a,
    {
        self.style_sheet = style_sheet;
        self
    }
}

/// The position of the tooltip. Defaults to following the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    /// The tooltip will follow the cursor.
    FollowCursor,
    /// The tooltip will appear on the top of the widget.
    Top,
    /// The tooltip will appear on the bottom of the widget.
    Bottom,
    /// The tooltip will appear on the left of the widget.
    Left,
    /// The tooltip will appear on the right of the widget.
    Right,
}

impl<'a, Message> Widget<Message> for Tooltip<'a, Message> {
    fn width(&self) -> Length {
        self.content.width()
    }

    fn height(&self) -> Length {
        self.content.height()
    }

    fn layout(
        &self,
        renderer: &dyn Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &dyn Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.content.widget.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut dyn Renderer,
        defaults: &renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let Self {
            content,
            position,
            gap,
            padding,
            style_sheet,
            ..
        } = self;

        let bounds = layout.bounds();

        if bounds.contains(cursor_position) {
            let gap = f32::from(*gap);
            let style = style_sheet.style();

            let defaults = renderer::Defaults {
                text: renderer::Text {
                    color: style.text_color.unwrap_or(defaults.text.color),
                },
            };

            let text_layout = Widget::<()>::layout(
                &self.tooltip,
                renderer,
                &layout::Limits::new(Size::ZERO, viewport.size())
                    .pad(Padding::new(*padding)),
            );

            let padding = f32::from(*padding);
            let text_bounds = text_layout.bounds();
            let x_center = bounds.x + (bounds.width - text_bounds.width) / 2.0;
            let y_center =
                bounds.y + (bounds.height - text_bounds.height) / 2.0;

            let mut tooltip_bounds = {
                let offset = match position {
                    Position::Top => Vector::new(
                        x_center,
                        bounds.y - text_bounds.height - gap - padding,
                    ),
                    Position::Bottom => Vector::new(
                        x_center,
                        bounds.y + bounds.height + gap + padding,
                    ),
                    Position::Left => Vector::new(
                        bounds.x - text_bounds.width - gap - padding,
                        y_center,
                    ),
                    Position::Right => Vector::new(
                        bounds.x + bounds.width + gap + padding,
                        y_center,
                    ),
                    Position::FollowCursor => Vector::new(
                        cursor_position.x,
                        cursor_position.y - text_bounds.height,
                    ),
                };

                Rectangle {
                    x: offset.x - padding,
                    y: offset.y - padding,
                    width: text_bounds.width + padding * 2.0,
                    height: text_bounds.height + padding * 2.0,
                }
            };

            if tooltip_bounds.x < viewport.x {
                tooltip_bounds.x = viewport.x;
            } else if viewport.x + viewport.width
                < tooltip_bounds.x + tooltip_bounds.width
            {
                tooltip_bounds.x =
                    viewport.x + viewport.width - tooltip_bounds.width;
            }

            if tooltip_bounds.y < viewport.y {
                tooltip_bounds.y = viewport.y;
            } else if viewport.y + viewport.height
                < tooltip_bounds.y + tooltip_bounds.height
            {
                tooltip_bounds.y =
                    viewport.y + viewport.height - tooltip_bounds.height;
            }

            renderer.begin_layer(*viewport);
            Widget::<()>::draw(
                &self.tooltip,
                renderer,
                &defaults,
                Layout::with_offset(
                    Vector::new(
                        tooltip_bounds.x + padding,
                        tooltip_bounds.y + padding,
                    ),
                    &text_layout,
                ),
                cursor_position,
                viewport,
            );
            renderer.end_layer();
        }

        self.content.draw(
            renderer,
            &defaults,
            layout,
            cursor_position,
            viewport,
        );
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.content.hash_layout(state);
    }
}

impl<'a, Message> From<Tooltip<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(column: Tooltip<'a, Message>) -> Element<'a, Message> {
        Element::new(column)
    }
}
