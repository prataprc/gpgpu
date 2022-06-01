use crate::{fonts, ColorTarget, Context, Extent, Resize, Result};

pub struct GlyphBox {
    attrs: Attributes,
    computed_attrs: Attributes,
    metrics: fonts::GlyphMetrics,
    computed_metrics: fonts::GlyphMetrics,
    // wgpu items
}

#[derive(Clone, Copy)]
pub struct Attributes {
    pub height: f32,
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            height: 0.0,
            fg: wgpu::Color::WHITE,
            bg: wgpu::Color::BLACK,
        }
    }
}

impl Resize for Attributes {
    fn resize(&self, _: Extent, scale_factor: Option<f32>) -> Self {
        match scale_factor {
            Some(scale_factor) => {
                Attributes { height: self.height * scale_factor, ..*self }
            }
            None => self.clone(),
        }
    }
}

impl GlyphBox {
    pub fn new(g: fonts::Glyph, attrs: Attributes) -> GlyphBox {
        let bb: fonts::GlyphRect = g
            .bounding_box()
            .map(|x| fonts::GlyphRect::from(x))
            .unwrap_or(fonts::GlyphRect::default());

        let metrics = fonts::GlyphMetrics {
            units_per_em: g.units_per_em() as f32,
            bounding_box: bb,
            hor_advance: g.hor_advance().map(|x| x as f32).unwrap_or(0.0),
            hor_side_bearing: g.hor_side_bearing().map(|x| x as f32).unwrap_or(0.0),
        };

        GlyphBox {
            attrs,
            computed_attrs: attrs,
            metrics,
            computed_metrics: metrics,
        }
    }

    pub fn print(&self, prefix: &str) {
        let Extent { width, height } = self.to_extent();
        println!("{}primv::GlyphBox({},{})", prefix, width, height);
    }
}

impl GlyphBox {
    pub fn to_extent(&self) -> Extent {
        let width = (self.metrics.bounding_box.to_width()
            / self.metrics.bounding_box.to_height())
            * self.attrs.height;
        Extent { width, height: self.attrs.height }
    }

    pub fn to_metrics(self) -> fonts::GlyphMetrics {
        self.metrics
    }

    pub fn resize(&mut self, extent: Extent, scale_factor: Option<f32>) -> &mut Self {
        if let Some(scale_factor) = scale_factor {
            self.computed_attrs = self.attrs.resize(extent, Some(scale_factor));
            self.computed_metrics = self.metrics.resize(extent, Some(scale_factor))
        }
        self
    }

    pub fn redraw(
        &mut self,
        _context: &Context,
        _encoder: &mut wgpu::CommandEncoder,
        _target: &mut ColorTarget,
    ) -> Result<()> {
        todo!()
    }
}
