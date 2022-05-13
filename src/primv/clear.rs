use crate::{ColorTarget, Context, Result};

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

impl Clear {
    pub fn redraw(
        &mut self,
        _context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        let ops = wgpu::Operations {
            load: wgpu::LoadOp::Clear(self.bg),
            store: true,
        };

        let mut render_pass = {
            let desc = wgpu::RenderPassDescriptor {
                label: Some("primv/clear:render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    resolve_target: None,
                    ops,
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };

        target.view_port.set_viewport(&mut render_pass);

        Ok(())
    }
}
