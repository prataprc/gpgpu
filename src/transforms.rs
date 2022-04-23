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

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,   // horizontal rotation
    pub pitch: Rad<f32>, // vertical rotation
}

impl Camera {
    pub fn new<P, Y, T>(position: P, yaw: Y, pitch: T) -> Camera
    where
        P: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        T: Into<Rad<f32>>,
    {
        Camera {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn to_dir(&self) -> Vector3<f32> {
        use cgmath::InnerSpace;

        Vector3::new(
            self.pitch.0.cos() * self.yaw.0.cos(),
            self.pitch.0.sin(),
            self.pitch.0.cos() * self.yaw.0.sin(),
        )
        .normalize()
    }
}

#[derive(Clone, Copy)]
pub struct Perspective<A>
where
    A: Into<Rad<f32>>,
{
    pub fov: A,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

#[derive(Clone, Copy)]
pub struct Ortho {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub near: f32,
    pub far: f32,
}

// Model, View, Projection transform
#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Projection {
    P(Matrix4<f32>),
    O(Matrix4<f32>),
}

impl Transforms {
    pub fn empty() -> Transforms {
        Transforms {
            translate: Matrix4::from_translation(Vector3::new(0.0, 0.0, 0.0)),
            rotate_x: Matrix4::from_angle_x(Rad(0.0)),
            rotate_y: Matrix4::from_angle_y(Rad(0.0)),
            rotate_z: Matrix4::from_angle_z(Rad(0.0)),
            scale: Matrix4::from_nonuniform_scale(1.0, 1.0, 1.0),
            view: Matrix4::one(),
            projection: Projection::P(Matrix4::one()),
        }
    }
}

impl Transforms {
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

    pub fn rotate_x_by<A>(&mut self, x: A) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        self.rotate_x = Matrix4::from_angle_x(x);
        self
    }

    pub fn rotate_y_by<A>(&mut self, y: A) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        self.rotate_y = Matrix4::from_angle_y(y);
        self
    }

    pub fn rotate_z_by<A>(&mut self, z: A) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        self.rotate_z = Matrix4::from_angle_z(z);
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
}

impl Transforms {
    pub fn look_at_rh<E, C, U>(&mut self, eye: E, center: C, up: U) -> &mut Self
    where
        E: Into<Point3<f32>>,
        C: Into<Point3<f32>>,
        U: Into<Vector3<f32>>,
    {
        self.view = Matrix4::look_at_rh(eye.into(), center.into(), up.into());
        self
    }

    pub fn look_to_rh<E, U>(&mut self, eye: E, camera: Camera, up: U) -> &mut Self
    where
        E: Into<Point3<f32>>,
        U: Into<Vector3<f32>>,
    {
        let dir = camera.to_dir();
        self.view = Matrix4::look_to_rh(eye.into(), dir, up.into());
        self
    }
}

impl Transforms {
    pub fn perspective_by<A>(&mut self, p: Perspective<A>) -> &mut Self
    where
        A: Into<Rad<f32>>,
    {
        let projection = cgmath::perspective(p.fov, p.aspect, p.near, p.far);
        self.projection = Projection::P(OPENGL_TO_WGPU_MATRIX * projection);
        self
    }

    pub fn orthogonal_by(&mut self, o: Ortho) -> &mut Self {
        let projection = cgmath::ortho(o.left, o.right, o.bottom, o.top, o.near, o.far);
        self.projection = Projection::O(OPENGL_TO_WGPU_MATRIX * projection);
        self
    }
}

impl Transforms {
    pub fn mvp(&self) -> Matrix4<f32> {
        let projection = match &self.projection {
            Projection::P(p) => p,
            Projection::O(p) => p,
        };
        projection * self.view * self.model()
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

impl UniformBuffer {
    const SIZE: usize = (16 * 4) + (16 * 4);
}

impl Transforms {
    pub fn to_bind_content(&self) -> Vec<u8> {
        let model = self.model();
        let mvp = self.mvp();

        let model_ref: &[f32; 16] = model.as_ref();
        let mvp_ref: &[f32; 16] = mvp.as_ref();
        let ub = UniformBuffer {
            model: model_ref.clone(),
            mvp: mvp_ref.clone(),
        };

        let contents: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
        contents.to_vec()
    }

    pub fn to_bind_group_layout_entry() -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
}
