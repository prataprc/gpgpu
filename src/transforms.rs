use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, One, Point3, Rad, Vector3};

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct Perspective<A>
where
    A: Into<Rad<f32>>,
{
    pub fov: A,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

pub struct Ortho {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

// Model, View, Projection transform
pub struct Transforms {
    // model
    scale: Matrix4<f32>,
    rotate_x: Matrix4<f32>,
    rotate_y: Matrix4<f32>,
    rotate_z: Matrix4<f32>,
    translate: Matrix4<f32>,
    // view
    view: Matrix4<f32>,
    // projector
    projection: Projection,
}

enum Projection {
    P(Matrix4<f32>),
    O(Matrix4<f32>),
}

impl Transforms {
    pub fn empty() -> Transforms {
        Transforms {
            translate: Matrix4::one(),
            rotate_x: Matrix4::one(),
            rotate_y: Matrix4::one(),
            rotate_z: Matrix4::one(),
            scale: Matrix4::one(),
            view: Matrix4::one(),
            projection: Projection::P(Matrix4::one()),
        }
    }

    pub fn translate_by(&mut self, shift: Vector3<f32>) -> &mut Self {
        self.translate = Matrix4::from_translation(shift);
        self
    }

    pub fn rotate_by<A>(&mut self, x: Option<A>, y: Option<A>, z: Option<A>) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        if let Some(z) = z {
            self.rotate_z = Matrix4::from_angle_z(z);
        }
        if let Some(y) = y {
            self.rotate_y = Matrix4::from_angle_y(y);
        }
        if let Some(x) = x {
            self.rotate_x = Matrix4::from_angle_x(x);
        }
        self
    }

    pub fn scale_by(&mut self, ratio: f32) -> &mut Self {
        self.scale = Matrix4::from_scale(ratio);
        self
    }

    pub fn scale_xyz_by(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.scale = Matrix4::from_nonuniform_scale(x, y, z);
        self
    }

    pub fn look_at_rh(
        &mut self,
        eye: Point3<f32>,
        center: Point3<f32>,
        up: Vector3<f32>,
    ) -> &mut Self {
        self.view = Matrix4::look_at_rh(eye, center, up);
        self
    }

    pub fn perspective_by<A>(&mut self, p: Perspective<A>) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        self.projection = {
            let mat = cgmath::perspective(p.fov, p.aspect, p.near, p.far);
            Projection::P(mat)
        };
        self
    }

    pub fn orthogonal_by(&mut self, o: Ortho) -> &mut Self {
        self.projection = {
            let mat = cgmath::ortho(o.left, o.right, o.bottom, o.top, o.near, o.far);
            Projection::O(mat)
        };
        self
    }

    pub fn projection(&self) -> Matrix4<f32> {
        let proj = match self.projection {
            Projection::P(proj) => OPENGL_TO_WGPU_MATRIX * proj,
            Projection::O(proj) => OPENGL_TO_WGPU_MATRIX * proj,
        };
        proj * self.view()
    }

    pub fn view(&self) -> Matrix4<f32> {
        self.view * self.model()
    }

    pub fn model(&self) -> Matrix4<f32> {
        self.translate * self.rotate_z * self.rotate_y * self.rotate_x * self.scale
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    model: [f32; 16],
    mvp: [f32; 16],
}

impl Transforms {
    pub fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("transform bind-group"),
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
        };
        device.create_bind_group_layout(&desc)
    }

    pub fn to_bind_group<A>(
        &self,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) -> wgpu::BindGroup {
        use wgpu::util::DeviceExt;

        let uniform = {
            let model = self.model();
            let model_ref: &[f32; 16] = model.as_ref();
            let mvp = self.projection();
            let mvp_ref: &[f32; 16] = mvp.as_ref();
            let ub = UniformBuffer {
                model: model_ref.clone(),
                mvp: mvp_ref.clone(),
            };
            let contents: [u8; 32 * 4] = bytemuck::cast(ub); // TODO: avoid hardcoding

            let desc = wgpu::util::BufferInitDescriptor {
                label: Some("transform-buffer"),
                contents: &contents,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            };
            device.create_buffer_init(&desc)
        };

        let desc = wgpu::BindGroupDescriptor {
            label: Some("transform-bind-group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        };
        device.create_bind_group(&desc)
    }
}
