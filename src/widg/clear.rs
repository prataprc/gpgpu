use crate::{widg, Result};

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
}

impl widg::Widget for Clear {
    fn render(
        &self,
        _: &widg::Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &widg::ColorTarget,
    ) -> Result<()> {
        let ops = wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.bg),
            store: true,
        };

        let desc = wgpu::RenderPassDescriptor {
            label: Some("widgets/clear:render-pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: target.view,
                resolve_target: None,
                ops,
            }],
            depth_stencil_attachment: None,
        };
        encoder.begin_render_pass(&desc);

        Ok(())
    }
}
