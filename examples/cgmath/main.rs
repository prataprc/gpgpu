use structopt::StructOpt;

use std::{any::type_name, fmt};

use gpgpu::util;

mod info;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(short = "t", default_value = "u32")]
    typ: String,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
enum SubCommand {
    Angle,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    match opts.subcmd {
        SubCommand::Angle => handle_angle(&opts),
        _ => {
            println!("### Absolute Difference");
            absolute_diff_eq::<u64>();
            absolute_diff_eq::<i64>();
            absolute_diff_eq::<usize>();
            absolute_diff_eq::<isize>();
            absolute_diff_eq::<f32>();
            absolute_diff_eq::<f64>();
            println!();

            println!("### Points");
            let p: cgmath::Point3<f32> = (0.5, 0.6, 0.7).into();
            let v = p.clone().to_homogeneous();
            println!("Homogenous vector for point {:?} : {:?}", p, v);
            println!();

            println!("### Vectors");
            println!(
                "Vector3::unix{{x, y, z}}: {:?} {:?} {:?}",
                cgmath::Vector3::<f32>::unit_x(),
                cgmath::Vector3::<f32>::unit_y(),
                cgmath::Vector3::<f32>::unit_z(),
            );
            let a = cgmath::Vector2::new(1, 2);
            let b = cgmath::Vector2::new(3, 4);
            println!(
                "Vector2 perpendicular dot product of {:?} . {:?} = {}",
                a,
                b,
                a.perp_dot(b)
            );
            println!();
        }
    }
}

fn handle_angle(opts: &Opt) {
    let rows = vec![info::AngleProperty::new_deg()];
    util::make_table(&rows).print_tty(!opts.no_color);

    let rows: Vec<info::TrigAngle> = (0..12)
        .map(|i| info::TrigAngle::new_deg(cgmath::Deg(30.0 * (i as f32))).into())
        .collect();
    util::make_table(&rows).print_tty(!opts.no_color);
}

fn absolute_diff_eq<T>()
where
    T: cgmath::AbsDiffEq,
    <T as cgmath::AbsDiffEq>::Epsilon: fmt::Display,
{
    let e = T::default_epsilon();
    println!("for `{}` default-epsilon: {}", type_name::<T>(), e);
}
