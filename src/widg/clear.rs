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
        encoder: &mut wgpu::CommandEncoder,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
        color_view: &wgpu::TextureView,
    ) -> Result<()> {
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
        encoder.begin_render_pass(&desc);

        Ok(())
    }
}
