use cgmath::{Angle, Deg};
use gpgpu::util::PrettyRow;
use prettytable::{cell, row};

pub struct AngleProperty {
    name: String,
    full_turn: String,
    turn_div_2: String,
    turn_div_3: String,
    turn_div_4: String,
    turn_div_6: String,
    bisect_2_3: String,
    bisect_3_2: String,
}

impl AngleProperty {
    pub fn new_deg() -> AngleProperty {
        let turn_div_2 = Deg::<f32>::turn_div_2();
        let turn_div_4 = Deg::<f32>::turn_div_4();
        let turn_div_3 = Deg::<f32>::turn_div_3();
        let turn_div_6 = Deg::<f32>::turn_div_6();

        AngleProperty {
            name: "deg".to_string(),
            full_turn: format!("{:?}", <Deg<f32> as Angle>::full_turn()),
            turn_div_2: format!("{:?}", turn_div_2),
            turn_div_3: format!("{:?}", turn_div_3),
            turn_div_4: format!("{:?}", turn_div_4),
            turn_div_6: format!("{:?}", turn_div_6),
            bisect_2_3: format!("{:?}", turn_div_2.bisect(turn_div_3)),
            bisect_3_2: format!("{:?}", turn_div_3.bisect(turn_div_2)),
        }
    }
}

impl PrettyRow for AngleProperty {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
        "Name", "turn_full", "turn_div_2", "turn_div_3", "turn_div_4",
        "turn_div_6", "bisect_2_3", "bisect_3_2"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.full_turn,
            self.turn_div_2,
            self.turn_div_3,
            self.turn_div_4,
            self.turn_div_6,
            self.bisect_2_3,
            self.bisect_3_2
        ]
    }
}

pub struct TrigAngle {
    angle: String,
    sin: String,
    cos: String,
    tan: String,
    sec: String,
    cosec: String,
    cotan: String,
    normalize: String,
    snormalize: String,
    opposite: String,
}

impl TrigAngle {
    pub fn new_deg(deg: Deg<f32>) -> TrigAngle {
        TrigAngle {
            angle: format!("{:?}", deg),
            sin: format!("{:?}", deg.clone().sin()),
            cos: format!("{:?}", deg.clone().cos()),
            tan: format!("{:?}", deg.clone().tan()),
            sec: format!("{:?}", deg.clone().sec()),
            cosec: format!("{:?}", deg.clone().csc()),
            cotan: format!("{:?}", deg.clone().cot()),
            normalize: format!("{:?}", deg.clone().normalize()),
            snormalize: format!("{:?}", deg.clone().normalize_signed()),
            opposite: format!("{:?}", deg.clone().opposite()),
        }
    }
}

impl PrettyRow for TrigAngle {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "Angle", "sin", "cos", "tan", "sec", "cosec", "cotan", "normalize",
            "snormalize", "opposite"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.angle,
            self.sin,
            self.cos,
            self.tan,
            self.sec,
            self.cosec,
            self.cotan,
            self.normalize,
            self.snormalize,
            self.opposite,
        ]
    }
}
