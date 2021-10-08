//! This example showcases a simple native custom widget that draws a circle.
mod triangle {
    use std::{cell::RefCell, sync::Arc};

    // For now, to implement a custom native widget you will need to add
    // `iced_native` and `iced_wgpu` to your dependencies.
    //
    // Then, you simply need to define your widget type and implement the
    // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
    //
    // Of course, you can choose to make the implementation renderer-agnostic,
    // if you wish to, by creating your own `Renderer` trait, which could be
    // implemented by `iced_wgpu` and other renderers.
    use iced_graphics::Primitive;
    use iced_native::{
        layout, mouse, Element, Hasher, Layout, Length, Point, Rectangle, Size,
        Widget,
    };
    use iced_wgpu::{
        wgpu::{self, util::DeviceExt},
        Renderer,
    };

    // Push constants might be a better option here,
    // but with uniform buffer we don't have to worry about the limit on some devices
    // if we were to expand this struct later.
    #[repr(C)]
    #[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
    struct TriangleUniforms {
        rotation: f32,
    }

    pub struct State {
        inner: RefCell<Option<StateInner>>,
        size: f32,
        rotation: f32,
    }

    // This is where all the rendering stuffs are.
    struct StateInner {
        _device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        pipeline: wgpu::RenderPipeline,
        bind_group: wgpu::BindGroup,
        uniform_buffer: wgpu::Buffer,
    }

    impl State {
        pub fn new() -> Self {
            Self {
                inner: RefCell::new(None),
                size: 0.0,
                rotation: 0.0,
            }
        }
    }

    impl StateInner {
        // Prepare the pipeline and bind groups and whatever.
        fn create(
            device: Arc<wgpu::Device>,
            queue: Arc<wgpu::Queue>,
            uniforms: &[TriangleUniforms],
        ) -> Self {
            let shader =
                device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: Some("shader"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("shader/shader.wgsl").into(),
                    ),
                });

            let uniform_buffer =
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    contents: bytemuck::cast_slice(uniforms),
                    usage: wgpu::BufferUsages::UNIFORM
                        | wgpu::BufferUsages::COPY_DST,
                    label: Some("Uniform Buffer"),
                });

            let bind_group_layout = device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("Uniform Buffer Bind Group Layout"),
                },
            );
            let bind_group =
                device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }],
                    label: Some("Uniform Buffer Bind Group"),
                });

            let pipeline_layout = device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: None,
                    push_constant_ranges: &[],
                    bind_group_layouts: &[&bind_group_layout],
                },
            );

            let pipeline = device.create_render_pipeline(
                &wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: "main",
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: "main",
                        targets: &[wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Bgra8UnormSrgb,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        }],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        front_face: wgpu::FrontFace::Ccw,
                        ..Default::default()
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                },
            );

            Self {
                _device: device,
                queue,
                pipeline,
                bind_group,
                uniform_buffer,
            }
        }
    }

    pub struct Triangle<'a> {
        state: &'a State,
    }

    impl<'a> Triangle<'a> {
        pub fn new(size: f32, rotation: f32, state: &'a mut State) -> Self {
            state.size = size;
            // If this component is being recreated,
            // it is possible that the rotation has changed,
            // so we compare with the previous state and update the uniform buffer if needed.
            if rotation != state.rotation {
                state.rotation = rotation;
                let borrow = state.inner.borrow();
                if let Some(inner) = &*borrow {
                    inner.queue.write_buffer(
                        &inner.uniform_buffer,
                        0,
                        bytemuck::cast_slice(&[TriangleUniforms { rotation }]),
                    );
                }
            }
            Self { state }
        }
    }

    impl<'a, Message> Widget<Message, Renderer> for Triangle<'a> {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(
                self.state.size * 2.0,
                self.state.size * 2.0,
            ))
        }

        fn hash_layout(&self, state: &mut Hasher) {
            use std::hash::Hash;

            self.state.size.to_bits().hash(state);
        }

        fn draw(
            &self,
            renderer: &mut Renderer,
            _defaults: &<Renderer as iced_native::Renderer>::Defaults,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) -> (Primitive<iced_wgpu::Backend>, mouse::Interaction) {
            let backend = renderer.backend();
            let device = backend.get_device();

            let mut borrow = self.state.inner.borrow_mut();
            // If the inner state, where the pipeline and rendering stuffs are,
            // has not been initialized, now is the time to do it.
            let inner: &mut StateInner = borrow.get_or_insert_with(|| {
                println!("Rebuilding pipeline. Don't do this often.");
                let queue = backend.get_queue();
                let r = StateInner::create(
                    device.clone(),
                    queue.clone(),
                    &[TriangleUniforms {
                        rotation: self.state.rotation,
                    }],
                );
                println!("Pipeline ready!");
                r
            });
            let pipeline = &inner.pipeline;
            let bounds = layout.bounds();
            let mut encoder = device.create_render_bundle_encoder(
                &wgpu::RenderBundleEncoderDescriptor {
                    label: Some("iced_wgpu::wgpu_area bundle_encoder"),
                    color_formats: &[backend.get_format()],
                    depth_stencil: None,
                    sample_count: 1,
                },
            );
            encoder.set_pipeline(&*pipeline);
            encoder.set_bind_group(0, &inner.bind_group, &[]);
            encoder.draw(0..3, 0..1);
            let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
                label: Some("Bundle"),
            });
            (
                // The Custom primitive takes a bundle and a rectangle bounds.
                Primitive::Custom(iced_wgpu::DirectWgpuJob::new(
                    bundle, bounds,
                )),
                mouse::Interaction::default(),
            )
        }
    }

    impl<'a, Message> Into<Element<'a, Message, Renderer>> for Triangle<'a> {
        fn into(self) -> Element<'a, Message, Renderer> {
            Element::new(self)
        }
    }
}

use iced::{executor, time, Application, Command, Settings};
use iced_native::{Alignment, Element, Length};
use iced_wgpu::{slider, Column, Container, Slider, Text};
use triangle::Triangle;

pub fn main() -> iced::Result {
    env_logger::init();
    Example::run(Settings::default())
}

struct Example {
    size: f32,
    speed: f32,
    angle: f32,
    size_slider: slider::State,
    speed_slider: slider::State,
    triangle_state: triangle::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
    SpeedChanged(f32),
    TimeTick,
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Example {
                size: 50.0,
                speed: 5.0,
                angle: 0.0,
                size_slider: slider::State::new(),
                speed_slider: slider::State::new(),
                triangle_state: triangle::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Direct wgpu access - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RadiusChanged(radius) => {
                self.size = radius;
            }
            Message::SpeedChanged(speed) => {
                self.speed = speed;
            }
            Message::TimeTick => {
                self.angle += self.speed / 100.0;
            }
        };
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        // Iced doesn't provide a good way to do animation yet. So we do this.
        time::every(std::time::Duration::from_millis(10))
            .map(|_| Message::TimeTick)
    }

    fn view(&mut self) -> Element<'_, Message, iced_wgpu::Renderer> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Alignment::Center)
            .push(Triangle::new(
                self.size,
                self.angle,
                &mut self.triangle_state,
            ))
            .push(Text::new(format!("Size: {:.2}", self.size)))
            .push(
                Slider::new(
                    &mut self.size_slider,
                    1.0..=100.0,
                    self.size,
                    Message::RadiusChanged,
                )
                .step(0.01),
            )
            .push(Text::new(format!("Speed: {:.2}", self.speed)))
            .push(
                Slider::new(
                    &mut self.speed_slider,
                    1.0..=10.0,
                    self.speed,
                    Message::SpeedChanged,
                )
                .step(0.01),
            );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
