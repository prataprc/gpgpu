use cgmath::{Deg, Matrix4, Point3, Vector2, Vector3, Vector4};
use structopt::StructOpt;

use std::{any::type_name, fmt, path, result};

use gpgpu::{
    err_at,
    util::{self, PrettyPrint},
    Error, Result,
};

mod info;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "force-color")]
    force_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
enum SubCommand {
    Angle,
    Info,
    Scale {
        #[structopt(long = "wireframe")]
        loc: Option<path::PathBuf>,

        #[structopt(short = "r", long = "ratio")]
        ratio: Option<f32>,

        #[structopt(long = "xyz", default_value = "1.0,1.0,1.0", use_delimiter = true)]
        xyz: Option<Vec<f32>>,
    },
    Translate {
        #[structopt(long = "wireframe")]
        loc: Option<path::PathBuf>,

        #[structopt(long = "xyz", default_value = "1.0,1.0,1.0", use_delimiter = true)]
        xyz: Vec<f32>,
    },
    Rotate {
        #[structopt(long = "wireframe")]
        loc: Option<path::PathBuf>,

        #[structopt(short = "x", default_value = "0.0")]
        x: f32,

        #[structopt(short = "y", default_value = "0.0")]
        y: f32,

        #[structopt(short = "z", default_value = "0.0")]
        z: f32,
    },
    Perspective {
        #[structopt(long = "wireframe")]
        loc: Option<path::PathBuf>,

        #[structopt(long = "args", use_delimiter = true)]
        args: Vec<f32>,
    },
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let res = match opts.subcmd {
        SubCommand::Angle => handle_angle(&opts),
        SubCommand::Info => handle_info(&opts),
        SubCommand::Scale { .. } => handle_scale(&opts),
        SubCommand::Translate { .. } => handle_translate(&opts),
        SubCommand::Rotate { .. } => handle_rotate(&opts),
        SubCommand::Perspective { .. } => handle_perspective(&opts),
    };

    res.map_err(|e| println!("Error {}", e)).ok();
}

fn handle_scale(opts: &Opt) -> Result<()> {
    use gpgpu::Transforms;

    let (loc, ratio, xyz) = match &opts.subcmd {
        SubCommand::Scale { loc, ratio, xyz } => (loc, ratio, xyz),
        _ => unreachable!(),
    };

    let mut transform = Transforms::empty();
    if let Some(ratio) = ratio {
        transform.scale_by(*ratio)
    } else if let Some([x]) = xyz.as_deref() {
        transform.scale_xyz_by(*x, 1.0, 1.0)
    } else if let Some([x, y]) = xyz.as_deref() {
        transform.scale_xyz_by(*x, *y, 1.0)
    } else if let Some([x, y, z]) = xyz.as_deref() {
        transform.scale_xyz_by(*x, *y, *z)
    } else {
        transform.scale_by(1.0)
    };

    match loc {
        Some(loc) => {
            let mut in_verts = load_from_file(loc)?;

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", in_verts.transform_mut(transform.model()))
        }
        None => {
            transform.model().print();
        }
    }

    Ok(())
}

fn handle_translate(opts: &Opt) -> Result<()> {
    use gpgpu::Transforms;

    let (loc, xyz) = match &opts.subcmd {
        SubCommand::Translate { loc, xyz } => (loc, xyz),
        _ => unreachable!(),
    };

    let mut transform = Transforms::empty();
    if let [x] = xyz.as_slice() {
        transform.translate_by((*x, 0.0, 0.0).into())
    } else if let [x, y] = xyz.as_slice() {
        transform.translate_by((*x, *y, 0.0).into())
    } else if let [x, y, z] = xyz.as_slice() {
        transform.translate_by((*x, *y, *z).into())
    } else {
        transform.translate_by((0.0, 0.0, 0.0).into())
    };

    match loc {
        Some(loc) => {
            let mut in_verts = load_from_file(loc)?;

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", in_verts.transform_mut(transform.model()));
        }
        None => transform.model().print(),
    }

    Ok(())
}

fn handle_rotate(opts: &Opt) -> Result<()> {
    use gpgpu::Transforms;

    let (loc, x, y, z) = match &opts.subcmd {
        SubCommand::Rotate { loc, x, y, z } => (loc, Deg(*x), Deg(*y), Deg(*z)),
        _ => unreachable!(),
    };

    let mut transform = Transforms::empty();
    transform.rotate_by(Some(x), Some(y), Some(z));

    match loc {
        Some(loc) => {
            let mut in_verts = load_from_file(loc)?;

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", in_verts.transform_mut(transform.model()));
        }
        None => {
            let matx = Matrix4::from_angle_x(x);
            let maty = Matrix4::from_angle_y(y);
            let matz = Matrix4::from_angle_z(z);

            println!("X-axis rotation");
            matx.print();
            println!();
            println!("Y-axis rotation");
            maty.print();
            println!();
            println!("Z-axis rotation");
            matz.print();
            println!();
            println!("XYZ rotation");
            transform.model().print();
            println!();
        }
    }

    Ok(())
}

fn handle_perspective(opts: &Opt) -> Result<()> {
    use gpgpu::{Perspective, Transforms};

    let (loc, fov, aspect, near, far) = match &opts.subcmd {
        SubCommand::Perspective { loc, args: a } => (loc, Deg(a[0]), a[1], a[2], a[3]),
        _ => unreachable!(),
    };

    let mut transform = Transforms::empty();
    transform.perspective_by(Perspective {
        fov,
        aspect,
        near,
        far,
    });

    match loc {
        Some(loc) => {
            let mut in_verts = load_from_file(loc)?;

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", in_verts.transform_mut(transform.mvp()));
        }
        None => {
            println!("Projection matrix");
            transform.mvp().print();
        }
    }

    Ok(())
}

fn handle_angle(opts: &Opt) -> Result<()> {
    let rows = vec![info::AngleProperty::new_deg()];
    util::make_table(&rows).print_tty(!opts.force_color);

    let rows: Vec<info::TrigAngle> = (0..12)
        .map(|i| info::TrigAngle::new_deg(Deg(30.0 * (i as f32))).into())
        .collect();
    util::make_table(&rows).print_tty(!opts.force_color);

    Ok(())
}

fn handle_info(_opts: &Opt) -> Result<()> {
    println!("### Absolute Difference");
    absolute_diff_eq::<u64>();
    absolute_diff_eq::<i64>();
    absolute_diff_eq::<usize>();
    absolute_diff_eq::<isize>();
    absolute_diff_eq::<f32>();
    absolute_diff_eq::<f64>();
    println!();

    println!("### Points");
    let p: Point3<f32> = (0.5, 0.6, 0.7).into();
    let v = p.clone().to_homogeneous();
    println!("Homogenous vector for point {:?} : {:?}", p, v);
    println!();

    println!("### Vectors");
    println!(
        "Vector3::unix{{x, y, z}}: {:?} {:?} {:?}",
        Vector3::<f32>::unit_x(),
        Vector3::<f32>::unit_y(),
        Vector3::<f32>::unit_z(),
    );
    let a = Vector2::new(1, 2);
    let b = Vector2::new(3, 4);
    println!(
        "Vector2 perpendicular dot product of {:?} . {:?} = {}",
        a,
        b,
        a.perp_dot(b)
    );
    println!();

    Ok(())
}

fn absolute_diff_eq<T>()
where
    T: cgmath::AbsDiffEq,
    <T as cgmath::AbsDiffEq>::Epsilon: fmt::Display,
{
    let e = T::default_epsilon();
    println!("for `{}` default-epsilon: {}", type_name::<T>(), e);
}

fn load_from_file<P>(loc: P) -> Result<Vertices>
where
    P: AsRef<path::Path>,
{
    use std::{fs, str::from_utf8};

    let data = err_at!(IOError, fs::read(loc))?;

    let txt = err_at!(IOError, from_utf8(&data))?;
    let mut vertices: Vec<Vertex> = vec![];
    for line in txt.lines() {
        Vertex::from_text_line(line)?.map(|v| vertices.push(v));
    }

    Ok(Vertices(vertices))
}

struct Vertices(Vec<Vertex>);

impl fmt::Display for Vertices {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let vertices = &self.0;
        for (i, v) in vertices.iter().enumerate() {
            write!(f, "({:4})=> {:?}\n", i, v.position)?;
        }
        Ok(())
    }
}

impl Vertices {
    fn transform_mut(&mut self, mat: Matrix4<f32>) -> &mut Self {
        self.0
            .iter_mut()
            .for_each(|v| v.position = (mat * Vector4::from(v.position)).into());
        self
    }
}

struct Vertex {
    position: [f32; 4],
    #[allow(dead_code)]
    color: [f32; 4],
}

impl Vertex {
    fn new(position: &[f32], color: &[f32]) -> Result<Vertex> {
        let position: [f32; 4] = match position {
            [x, y, z] => Point3::from((*x, *y, *z)).to_homogeneous().into(),
            [x, y] => Point3::from((*x, *y, 0.0)).to_homogeneous().into(),
            _ => err_at!(Invalid, msg: "invalid position {:?}", position)?,
        };
        let color: [f32; 4] = match color {
            [r, g, b, a] => [*r, *g, *b, *a],
            [r, g, b] => [*r, *g, *b, 1.0],
            [] => [1.0, 1.0, 1.0, 1.0],
            _ => err_at!(Invalid, msg: "invalid color {:?}", color)?,
        };

        Ok(Vertex { position, color })
    }

    fn from_text_line(txt: &str) -> Result<Option<Vertex>> {
        match txt.split(";").collect::<Vec<&str>>().as_slice() {
            ["", ""] | [""] | [] => Ok(None),
            [pos, ""] => Some(Vertex::from_position(&util::parse_csv(pos)?)).transpose(),
            [pos, color] => Some(Vertex::new(
                &util::parse_csv(pos)?,
                &util::parse_csv(color)?,
            ))
            .transpose(),
            [pos] => Some(Vertex::from_position(&util::parse_csv(pos)?)).transpose(),
            _ => err_at!(Invalid, msg: "invalid vertex {}", txt),
        }
    }

    fn from_position(position: &[f32]) -> Result<Vertex> {
        Self::new(position, &[1.0, 1.0, 1.0, 1.0])
    }
}
