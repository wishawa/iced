use std::rc::Rc;

use iced_graphics::Rectangle;

/// A render job containing [`wgpu::RenderBundle`] to describe what to render
/// and [`Rectangle`] to describe where to render it.
#[derive(Clone, Debug)]
pub struct DirectWgpuJob {
    bundle: Rc<wgpu::RenderBundle>,
    bounds: Rectangle,
}

impl DirectWgpuJob {
    /// Create a new [`DirectWgpuJob`] that will execute the given [`wgpu::RenderBundle`].
    /// See the `direct_wgpu` example for usage.
    pub fn new(bundle: wgpu::RenderBundle, bounds: Rectangle) -> Self {
        Self::new_rc(Rc::new(bundle), bounds)
    }
    /// Create a new [`DirectWgpuJob`] that will execute the given [`wgpu::RenderBundle`].
    /// [`DirectWgpuJob`] internally stores the Bundle in an [`Rc`],
    /// so if you already have it in `Rc` we don't have to create a new one.
    pub fn new_rc(bundle: Rc<wgpu::RenderBundle>, bounds: Rectangle) -> Self {
        Self { bundle, bounds }
    }
}

#[derive(Debug)]
pub struct Pipeline;

impl Pipeline {
    pub fn new() -> Self {
        Self
    }
    pub fn draw(
        &mut self,
        _device: &wgpu::Device,
        _staging_belt: &mut wgpu::util::StagingBelt,
        encoder: &mut wgpu::CommandEncoder,
        jobs: &[&DirectWgpuJob],
        target: &wgpu::TextureView,
        _scale: f32,
    ) {
        for job in jobs {
            let DirectWgpuJob { bundle, bounds } = job;
            let mut render_pass =
                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: target,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            render_pass.set_viewport(
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                0.0,
                1.0,
            );
            render_pass.execute_bundles(std::iter::once(&**bundle));
        }
    }
}
