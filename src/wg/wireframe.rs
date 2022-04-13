use cgmath::{Matrix4, One, Point3, Rad, Vector3, Vector4};
use std::{fmt, path, result};

use crate::{Error, Result};

#[derive(Clone)]
pub enum Vertices {
    Lines { list: Vec<Vector4<f32>> },
}

impl fmt::Display for Vertices {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Vertices::Lines { list } => {
                for (i, v) in list.iter().enumerate() {
                    write!(f, "({:4})=> {:?}\n", i, v)?;
                }
            }
        }

        Ok(())
    }
}

impl Vertices {
    pub fn from_file<P>(loc: P) -> Result<Vertices>
    where
        P: AsRef<path::Path>,
    {
        use std::fs;

        let data = err_at!(IOError, fs::read(loc))?;
        Self::from_bytes(&data)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Vertices> {
        use std::str::from_utf8;

        let txt = err_at!(IOError, from_utf8(data))?;
        let mut list: Vec<Vector4<f32>> = vec![];
        for line in txt.lines() {
            let mut coord = [0_f32; 3];
            for (i, item) in line.split(",").take(3).enumerate() {
                coord[i] = err_at!(IOError, item.parse())?;
            }
            list.push(Point3::from(coord).to_homogeneous());
        }
        println!("from_bytes {:?}", list);

        Ok(Vertices::Lines { list })
    }
}

// Model transformations
impl Vertices {
    pub fn translate(&self, shift: Vector3<f32>) -> Self {
        let mat = Matrix4::from_translation(shift);
        match self {
            Vertices::Lines { list } => Vertices::Lines {
                list: list.iter().map(|v| mat * *v).collect(),
            },
        }
    }

    pub fn rotate<A>(&self, x: Option<A>, y: Option<A>, z: Option<A>) -> Self
    where
        A: Into<Rad<f32>>,
    {
        let mut mat = Matrix4::one();
        if let Some(z) = z {
            mat = mat * Matrix4::from_angle_z(z);
        }
        if let Some(y) = y {
            mat = mat * Matrix4::from_angle_y(y);
        }
        if let Some(x) = x {
            mat = mat * Matrix4::from_angle_x(x);
        }

        match self {
            Vertices::Lines { list } => Vertices::Lines {
                list: list.iter().map(|v| mat * *v).collect(),
            },
        }
    }

    pub fn scale(&self, ratio: f32) -> Self {
        let mat = Matrix4::from_scale(ratio);
        match self {
            Vertices::Lines { list } => Vertices::Lines {
                list: list.iter().map(|v| mat * *v).collect(),
            },
        }
    }

    pub fn scale_xyz(&self, x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        let mat = Matrix4::from_nonuniform_scale(x_scale, y_scale, z_scale);
        match self {
            Vertices::Lines { list } => Vertices::Lines {
                list: list.iter().map(|v| mat * *v).collect(),
            },
        }
    }

    pub fn transform(&self, mat: Matrix4<f32>) -> Self {
        match self {
            Vertices::Lines { list } => Vertices::Lines {
                list: list.iter().map(|v| mat * *v).collect(),
            },
        }
    }
}
