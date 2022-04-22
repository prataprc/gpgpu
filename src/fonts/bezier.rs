pub fn binomial_coeffs(n: usize) -> Vec<f32> {
    let n = match n {
        0 => return vec![1.0],
        1 => return vec![1.0, 1.0],
        n => n,
    };

    let mut levels: Vec<Vec<f32>> = vec![vec![1.0, 1.0]];

    for k in 2..=n {
        let prev = levels.pop().unwrap();
        let mut next = vec![1.0];
        (0..(k - 1)).for_each(|j| next.push(prev[j] + prev[j + 1]));
        next.push(1.0);
        levels.push(next)
    }

    levels.pop().unwrap()
}

pub fn bezier2(t: f32, points: [cgmath::Point2<f32>; 3]) -> cgmath::Point2<f32> {
    use cgmath::Matrix;

    let coeff: cgmath::Matrix3<f32> =
        [[1.0, -2.0, 1.0], [0.0, 2.0, -2.0], [0.0, 0.0, 1.0]].into();

    let bases = cgmath::Matrix3::<f32>::from_cols(
        cgmath::Vector3::<f32>::from([1.0, t.powf(1.0), t.powf(2.0)]),
        [0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
    )
    .transpose();

    let x_matrix = cgmath::Matrix3::<f32>::from_cols(
        cgmath::Vector3::<f32>::from([points[0].x, points[1].x, points[2].x]),
        [0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
    );
    let y_matrix = cgmath::Matrix3::<f32>::from_cols(
        cgmath::Vector3::<f32>::from([points[0].y, points[1].y, points[2].y]),
        [0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0].into(),
    );

    let x = (bases * coeff) * x_matrix;
    let y = (bases * coeff) * y_matrix;

    (x.x.x, y.x.x).into()
}

pub fn bezier3(t: f32, points: [cgmath::Point2<f32>; 4]) -> cgmath::Point2<f32> {
    use cgmath::Matrix;

    let coeff: cgmath::Matrix4<f32> = [
        [1.0, -3.0, 3.0, -1.0],
        [0.0, 3.0, -6.0, 3.0],
        [0.0, 0.0, 3.0, -3.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
    .into();

    let bases = cgmath::Matrix4::<f32>::from_cols(
        cgmath::Vector4::<f32>::from([1.0, t.powf(1.0), t.powf(2.0), t.powf(3.0)]),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
    )
    .transpose();

    let x_matrix = cgmath::Matrix4::<f32>::from_cols(
        cgmath::Vector4::<f32>::from([
            points[0].x,
            points[1].x,
            points[2].x,
            points[3].x,
        ]),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
    );
    let y_matrix = cgmath::Matrix4::<f32>::from_cols(
        cgmath::Vector4::<f32>::from([
            points[0].y,
            points[1].y,
            points[2].y,
            points[3].y,
        ]),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
        [0.0, 0.0, 0.0, 0.0].into(),
    );

    let x = (bases * coeff) * x_matrix;
    let y = (bases * coeff) * y_matrix;

    (x.x.x, y.x.x).into()
}

pub fn r_bezier2(
    t: f32,
    points: [cgmath::Point2<f32>; 3],
    ratios: [f32; 3],
) -> cgmath::Point2<f32> {
    const Q: usize = 2;

    let xs: [f32; 3] = [points[0].x, points[1].x, points[2].x];
    let ys: [f32; 3] = [points[0].y, points[1].y, points[2].y];

    let denom: Vec<f32> = binomial_coeffs(Q)
        .into_iter()
        .zip(ratios.iter())
        .enumerate()
        .map(|(k, (bc, r))| bc * (1.0 - t).powf((Q - k) as f32) * t.powf(k as f32) * r)
        .collect();

    let x: f32 = denom
        .clone()
        .into_iter()
        .zip(xs.into_iter())
        .map(|(x, p)| x * p)
        .sum::<f32>()
        / denom.iter().sum::<f32>();

    let y: f32 = denom
        .clone()
        .into_iter()
        .zip(ys.into_iter())
        .map(|(y, p)| y * p)
        .sum::<f32>()
        / denom.iter().sum::<f32>();

    cgmath::Point2::from((x, y))
}

pub fn r_bezier3(
    t: f32,
    points: [cgmath::Point2<f32>; 4],
    ratios: [f32; 4],
) -> cgmath::Point2<f32> {
    const Q: usize = 3;

    let xs: [f32; 4] = [points[0].x, points[1].x, points[2].x, points[3].x];
    let ys: [f32; 4] = [points[0].y, points[1].y, points[2].y, points[3].y];

    let denom: Vec<f32> = binomial_coeffs(Q)
        .into_iter()
        .zip(ratios.iter())
        .enumerate()
        .map(|(k, (bc, r))| bc * (1.0 - t).powf((Q - k) as f32) * t.powf(k as f32) * r)
        .collect();

    let x: f32 = denom
        .clone()
        .into_iter()
        .zip(xs.into_iter())
        .map(|(x, p)| x * p)
        .sum::<f32>()
        / denom.iter().sum::<f32>();

    let y: f32 = denom
        .clone()
        .into_iter()
        .zip(ys.into_iter())
        .map(|(y, p)| y * p)
        .sum::<f32>()
        / denom.iter().sum::<f32>();

    cgmath::Point2::from((x, y))
}

#[cfg(test)]
#[path = "bezier_test.rs"]
mod bezier_test;
