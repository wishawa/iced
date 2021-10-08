use iced_native::{
    image, svg, Background, Color, Font, Rectangle, Size, Vector,
};

use crate::alignment;
use crate::triangle;
use crate::Backend;

use std::sync::Arc;

pub trait PrimitiveBackend {
    type CustomRenderPrimitive;
}

impl<B: Backend> PrimitiveBackend for B {
    type CustomRenderPrimitive = B::CustomRenderPrimitive;
}

/// A rendering primitive.
#[derive(Debug, Clone)]
pub enum Primitive<B: PrimitiveBackend> {
    /// An empty primitive
    None,
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Primitive<B>>,
    },
    /// A text primitive
    Text {
        /// The contents of the text
        content: String,
        /// The bounds of the text
        bounds: Rectangle,
        /// The color of the text
        color: Color,
        /// The size of the text
        size: f32,
        /// The font of the text
        font: Font,
        /// The horizontal alignment of the text
        horizontal_alignment: alignment::Horizontal,
        /// The vertical alignment of the text
        vertical_alignment: alignment::Vertical,
    },
    /// A quad primitive
    Quad {
        /// The bounds of the quad
        bounds: Rectangle,
        /// The background of the quad
        background: Background,
        /// The border radius of the quad
        border_radius: f32,
        /// The border width of the quad
        border_width: f32,
        /// The border color of the quad
        border_color: Color,
    },
    /// An image primitive
    Image {
        /// The handle of the image
        handle: image::Handle,
        /// The bounds of the image
        bounds: Rectangle,
    },
    /// An SVG primitive
    Svg {
        /// The path of the SVG file
        handle: svg::Handle,

        /// The bounds of the viewport
        bounds: Rectangle,
    },
    /// A clip primitive
    Clip {
        /// The bounds of the clip
        bounds: Rectangle,
        /// The offset transformation of the clip
        offset: Vector<u32>,
        /// The content of the clip
        content: Box<Primitive<B>>,
    },
    /// A primitive that applies a translation
    Translate {
        /// The translation vector
        translation: Vector,

        /// The primitive to translate
        content: Box<Primitive<B>>,
    },
    /// A low-level primitive to render a mesh of triangles.
    ///
    /// It can be used to render many kinds of geometry freely.
    Mesh2D {
        /// The vertex and index buffers of the mesh
        buffers: triangle::Mesh2D,

        /// The size of the drawable region of the mesh.
        ///
        /// Any geometry that falls out of this region will be clipped.
        size: Size,
    },
    /// A cached primitive.
    ///
    /// This can be useful if you are implementing a widget where primitive
    /// generation is expensive.
    Cached {
        /// The cached primitive
        cache: Arc<Primitive<B>>,
    },
    /// A rendering job specific to the backend. Currently used in `iced_wgpu` to allow rendering with wgpu directly.
    Custom(B::CustomRenderPrimitive),
}

impl<B: PrimitiveBackend> Default for Primitive<B> {
    fn default() -> Self {
        Self::None
    }
}

impl PrimitiveBackend for () {
    type CustomRenderPrimitive = ();
}

impl<B: Backend> From<Primitive<()>> for Primitive<B> {
    fn from(from: Primitive<()>) -> Self {
        match from {
            Primitive::None => Primitive::None,
            Primitive::Group { primitives } => Primitive::Group {
                primitives: primitives.into_iter().map(From::from).collect(),
            },
            Primitive::Text {
                content,
                bounds,
                color,
                size,
                font,
                horizontal_alignment,
                vertical_alignment,
            } => Primitive::Text {
                content,
                bounds,
                color,
                size,
                font,
                horizontal_alignment,
                vertical_alignment,
            },
            Primitive::Quad {
                bounds,
                background,
                border_radius,
                border_width,
                border_color,
            } => Primitive::Quad {
                bounds,
                background,
                border_radius,
                border_width,
                border_color,
            },
            Primitive::Image { handle, bounds } => {
                Primitive::Image { handle, bounds }
            }
            Primitive::Svg { handle, bounds } => {
                Primitive::Svg { handle, bounds }
            }
            Primitive::Clip {
                bounds,
                offset,
                content,
            } => Primitive::Clip {
                bounds,
                offset,
                content: Box::new(From::from(*content)),
            },
            Primitive::Translate {
                translation,
                content,
            } => Primitive::Translate {
                translation,
                content: Box::new(From::from(*content)),
            },
            Primitive::Mesh2D { buffers, size } => {
                Primitive::Mesh2D { buffers, size }
            }
            Primitive::Cached { cache } => Primitive::Cached {
                cache: Arc::new(From::from((*cache).clone())),
            },
            Primitive::Custom(_) => {
                unreachable!("Canvas does not draw backend-specific primitives")
            }
        }
    }
}
