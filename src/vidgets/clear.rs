use crate::Result;

pub struct Clear {
    bg: wgpu::Color,
}

impl Clear {
    pub fn new<C>(bg: C) -> Clear
    where
        C: Into<wgpu::Color>,
    {
        Clear { bg: bg.into() }
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_view: &wgpu::TextureView,
    ) -> Result<()> {
        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("widgets/clear:encoder"),
            };
            device.create_command_encoder(&desc)
        };
        {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(self.bg),
                store: true,
            };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("widgets/clear:render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: color_view,
                    resolve_target: None,
                    ops,
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };

        let cmd_buffers = vec![encoder.finish()];
        queue.submit(cmd_buffers.into_iter());

        Ok(())
    }
}
