pub mod circle;
pub mod clear;
pub mod load;
// mod wireframe;

use crate::{Result, Transforms};

pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub struct Context<'a> {
    pub transforms: &'a Transforms,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

pub struct ColorTarget<'a> {
    pub size: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
    pub view: &'a wgpu::TextureView,
}

pub trait Widget {
    fn render(
        &self,
        _: &Context,
        _: &mut wgpu::CommandEncoder,
        _: &ColorTarget,
    ) -> Result<()>;
}
