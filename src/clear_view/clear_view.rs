pub struct ClearView;

impl ClearView {
    pub fn render<C>(
        &self,
        color: C,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_view: &wgpu::TextureView,
    ) where
        C: Into<wgpu::Color>,
    {
        let color: wgpu::Color = color.into();
        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("clear-view"),
            };
            device.create_command_encoder(&desc)
        };
        {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: true,
            };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("clear-view-render-pass"),
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
    }
}
