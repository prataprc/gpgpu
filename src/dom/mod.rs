mod style;

pub use style::{ComputedStyle, Display, FlowAlign, Style, StyleBorder, StyleValue};

pub struct Page {
    root: Node,
}

pub struct Node {
    geom: Geometry,
    style: Style,
    node: InnerNode,
    children: Vec<Node>,
}

pub enum InnerNode {
    None,
}

impl InnerNode {
    fn to_name(&self) -> String {
        match self {
            InnerNode::None => "none",
        }
        .to_string()
    }
}

struct Constraint {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

struct Geometry {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    d: f32,
}

struct Layout {
    constraint_stack: Vec<Constraint>,
    constraint: Constraint,
}

fn do_layout(node: &Node, layout: &mut Layout) {
    //
}
