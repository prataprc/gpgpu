mod circle;
mod clear;
mod load;
mod save;
mod wireframe;

pub use circle::Circle;
pub use clear::Clear;
pub use load::Load;
pub use save::Save;
pub use wireframe::Wireframe;

pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
