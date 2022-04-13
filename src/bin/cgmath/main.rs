use cgmath::{Deg, Matrix4, Point3, Vector2, Vector3};
use structopt::StructOpt;

use std::{any::type_name, fmt, path};

use gpgpu::{
    util::{self, PrettyPrint},
    Result,
};

mod info;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
enum SubCommand {
    Angle,
    Info,
    Scale {
        #[structopt(long = "vertices")]
        loc: Option<path::PathBuf>,

        #[structopt(short = "r", long = "ratio")]
        ratio: Option<f32>,

        #[structopt(long = "xyz", default_value = "1.0,1.0,1.0", use_delimiter = true)]
        xyz: Option<Vec<f32>>,
    },
    Translate {
        #[structopt(long = "vertices")]
        loc: Option<path::PathBuf>,

        #[structopt(long = "xyz", default_value = "1.0,1.0,1.0", use_delimiter = true)]
        xyz: Vec<f32>,
    },
    Rotate {
        #[structopt(long = "vertices")]
        loc: Option<path::PathBuf>,

        #[structopt(short = "x", default_value = "0.0")]
        x: f32,

        #[structopt(short = "y", default_value = "0.0")]
        y: f32,

        #[structopt(short = "z", default_value = "0.0")]
        z: f32,
    },
    Frustum {
        #[structopt(long = "vertices")]
        loc: Option<path::PathBuf>,

        #[structopt(long = "args", use_delimiter = true)]
        args: Vec<f32>,
    },
    Projection {
        #[structopt(long = "vertices")]
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
        SubCommand::Frustum { .. } => handle_frustum(&opts),
        SubCommand::Projection { .. } => handle_projection(&opts),
    };

    res.map_err(|e| println!("Error {}", e)).ok();
}

fn handle_scale(opts: &Opt) -> Result<()> {
    use gpgpu::wg::wireframe::Vertices;

    let (loc, ratio, xyz) = match &opts.subcmd {
        SubCommand::Scale { loc, ratio, xyz } => (loc, ratio, xyz),
        _ => unreachable!(),
    };

    match loc {
        Some(loc) => {
            let in_verts = Vertices::from_file(loc)?;

            let out_verts = if let Some(ratio) = ratio {
                in_verts.scale(*ratio)
            } else if let Some([x]) = xyz.as_deref() {
                in_verts.scale_xyz(*x, 1.0, 1.0)
            } else if let Some([x, y]) = xyz.as_deref() {
                in_verts.scale_xyz(*x, *y, 1.0)
            } else if let Some([x, y, z]) = xyz.as_deref() {
                in_verts.scale_xyz(*x, *y, *z)
            } else {
                in_verts.scale(1.0)
            };

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", out_verts);
        }
        None => {
            let mat = if let Some(ratio) = ratio {
                Matrix4::from_scale(*ratio)
            } else if let Some([x]) = xyz.as_deref() {
                Matrix4::from_nonuniform_scale(*x, 1.0, 1.0)
            } else if let Some([x, y]) = xyz.as_deref() {
                Matrix4::from_nonuniform_scale(*x, *y, 1.0)
            } else if let Some([x, y, z]) = xyz.as_deref() {
                Matrix4::from_nonuniform_scale(*x, *y, *z)
            } else {
                Matrix4::from_scale(1.0)
            };

            mat.print();
        }
    }

    Ok(())
}

fn handle_translate(opts: &Opt) -> Result<()> {
    use gpgpu::wg::wireframe::Vertices;

    let (loc, xyz) = match &opts.subcmd {
        SubCommand::Translate { loc, xyz } => (loc, xyz),
        _ => unreachable!(),
    };

    match loc {
        Some(loc) => {
            let in_verts = Vertices::from_file(loc)?;

            let out_verts = if let [x] = xyz.as_slice() {
                in_verts.translate((*x, 0.0, 0.0).into())
            } else if let [x, y] = xyz.as_slice() {
                in_verts.translate((*x, *y, 0.0).into())
            } else if let [x, y, z] = xyz.as_slice() {
                in_verts.translate((*x, *y, *z).into())
            } else {
                in_verts.translate((0.0, 0.0, 0.0).into())
            };

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", out_verts);
        }
        None => {
            let tv: Vector3<f32> = if let [x] = xyz.as_slice() {
                (*x, 0.0, 0.0).into()
            } else if let [x, y] = xyz.as_slice() {
                (*x, *y, 0.0).into()
            } else if let [x, y, z] = xyz.as_slice() {
                (*x, *y, *z).into()
            } else {
                (0.0, 0.0, 0.0).into()
            };
            let mat = Matrix4::from_translation(tv);
            mat.print()
        }
    }

    Ok(())
}

fn handle_rotate(opts: &Opt) -> Result<()> {
    use gpgpu::wg::wireframe::Vertices;

    let (loc, x, y, z) = match &opts.subcmd {
        SubCommand::Rotate { loc, x, y, z } => (loc, Deg(*x), Deg(*y), Deg(*z)),
        _ => unreachable!(),
    };

    match loc {
        Some(loc) => {
            let in_verts = Vertices::from_file(loc)?;
            let out_verts = in_verts.rotate(Some(x), Some(y), Some(z));

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", out_verts);
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
        }
    }

    Ok(())
}

fn handle_frustum(opts: &Opt) -> Result<()> {
    use gpgpu::wg::wireframe::Vertices;

    let (loc, frustum) = match &opts.subcmd {
        SubCommand::Frustum { loc, args: a } => {
            let frustum = cgmath::frustum(a[0], a[1], a[2], a[3], a[4], a[5]);
            (loc, frustum)
        }
        _ => unreachable!(),
    };

    match loc {
        Some(loc) => {
            let in_verts = Vertices::from_file(loc)?;
            let out_verts = in_verts.transform(frustum);

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", out_verts);
        }
        None => {
            println!("Frustum matrix");
            frustum.print();
        }
    }

    Ok(())
}

fn handle_projection(opts: &Opt) -> Result<()> {
    use gpgpu::wg::wireframe::Vertices;

    let (loc, perspective) = match &opts.subcmd {
        SubCommand::Projection { loc, args: a } => {
            let perspective = cgmath::perspective(Deg(a[0]), a[1], a[2], a[3]);
            (loc, perspective)
        }
        _ => unreachable!(),
    };

    match loc {
        Some(loc) => {
            let in_verts = Vertices::from_file(loc)?;
            let out_verts = in_verts.transform(perspective);

            println!("Input");
            println!("{}", in_verts);

            println!();

            println!("Output");
            println!("{}", out_verts);
        }
        None => {
            println!("Projection matrix");
            perspective.print();
        }
    }

    Ok(())
}

fn handle_angle(opts: &Opt) -> Result<()> {
    let rows = vec![info::AngleProperty::new_deg()];
    util::make_table(&rows).print_tty(!opts.no_color);

    let rows: Vec<info::TrigAngle> = (0..12)
        .map(|i| info::TrigAngle::new_deg(Deg(30.0 * (i as f32))).into())
        .collect();
    util::make_table(&rows).print_tty(!opts.no_color);

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
