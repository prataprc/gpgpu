use log::trace;

// pub mod line;
// pub mod div;
pub mod shape;
pub mod win;

use crate::{
    BoxLayout, ColorTarget, Context, Error, Result, Size, State, Style, Viewport,
};

macro_rules! dispatch {
    (call, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => val.$($toks)*,
            Node::Shape(val) => val.$($toks)*,
            //Node::Div(val) => val.$($toks)*,
        }
    };
    (get_state, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => {
                let state: &State<_> = val.as_ref();
                &state.$($toks)*
            },
            Node::Shape(val) => {
                let state: &State<_> = val.as_ref();
                &state.$($toks)*
            },
            //Node::Div(val) => {
            //    let state: &State<_> = val.as_ref();
            //    &state.$($toks)*
            //},
        }
    };
    (set_state, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => {
                let state: &mut State<_> = val.as_mut();
                &mut state.$($toks)*
            }
            Node::Shape(val) => {
                let state: &mut State<_> = val.as_mut();
                &mut state.$($toks)*
            }
            //Node::Div(val) => {
            //    let state: &mut State<_> = val.as_mut();
            //    &mut state.$($toks)*
            //}
        }
    };
}

pub trait Domesticate {
    fn to_mut_children(&mut self) -> Option<&mut Vec<Node>>;

    fn to_extent(&self) -> Size;

    fn resize(&mut self, size: Size);

    fn scale_factor_changed(&mut self, scale_factor: f32);

    fn to_viewport(&self) -> Viewport;

    fn redraw(
        &mut self,
        _: &Context,
        _: &mut wgpu::CommandEncoder,
        _: &mut ColorTarget,
    ) -> Result<()>;
}

pub enum Node {
    Win(win::Win),
    Shape(shape::Shape),
    // Div(div::Div),
}

impl From<win::Win> for Node {
    fn from(val: win::Win) -> Node {
        Node::Win(val)
    }
}

impl From<shape::Shape> for Node {
    fn from(val: shape::Shape) -> Node {
        Node::Shape(val)
    }
}

//impl From<div::Div> for Node {
//    fn from(val: div::Div) -> Node {
//        Node::Div(val)
//    }
//}

impl Node {
    fn as_computed_style(&self) -> &Style {
        dispatch!(get_state, self, computed_style)
    }

    fn to_flex_node(&self) -> stretch::node::Node {
        dispatch!(get_state, self, flex_node).clone().unwrap()
    }

    fn set_flex_node(&mut self, flex_node: stretch::node::Node) {
        let p = dispatch!(set_state, self, flex_node);
        *p = Some(flex_node);
    }

    fn set_box_layout(&mut self, box_layout: BoxLayout) {
        let p = dispatch!(set_state, self, box_layout);
        *p = box_layout;
    }

    fn print(&self, prefix: &str) {
        dispatch!(call, self, print(prefix))
    }
}

impl Domesticate for Node {
    fn to_mut_children(&mut self) -> Option<&mut Vec<Node>> {
        dispatch!(call, self, to_mut_children())
    }

    fn to_extent(&self) -> Size {
        dispatch!(call, self, to_extent())
    }

    fn resize(&mut self, size: Size) {
        dispatch!(call, self, resize(size))
    }

    fn scale_factor_changed(&mut self, scale_factor: f32) {
        dispatch!(call, self, scale_factor_changed(scale_factor))
    }

    fn to_viewport(&self) -> Viewport {
        dispatch!(call, self, to_viewport())
    }

    fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        use std::mem;

        let view_port = {
            let view_port = dispatch!(call, self, to_viewport());
            mem::replace(&mut target.view_port, view_port)
        };
        dispatch!(call, self, redraw(context, encoder, target))?;
        let _view_port = mem::replace(&mut target.view_port, view_port);
        Ok(())
    }
}

pub struct Dom {
    root: Node,
}

impl Dom {
    pub fn resize(&mut self, size: Size) {
        self.root.resize(size)
    }

    pub fn scale_factor_changed(&mut self, scale_factor: f32) {
        self.root.scale_factor_changed(scale_factor)
    }

    pub fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        self.root.redraw(context, encoder, target)
    }
}

impl Dom {
    pub fn new(win: win::Win) -> Dom {
        let root = Node::Win(win);
        Dom { root: root }
    }

    pub fn compute_layout(
        &mut self,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Result<()> {
        let mut flex = stretch::node::Stretch::new();
        build_layout(&mut flex, &mut self.root)?;

        let size = stretch::geometry::Size {
            width: match width {
                Some(w) => stretch::number::Number::Defined(w),
                None => stretch::number::Number::Undefined,
            },
            height: match height {
                Some(h) => stretch::number::Number::Defined(h),
                None => stretch::number::Number::Undefined,
            },
        };
        let flex_node = self.root.to_flex_node();
        err_at!(Invalid, flex.compute_layout(flex_node, size))?;
        gather_layout(&flex, &mut self.root)
    }

    pub fn print(&self) {
        self.root.print("")
    }
}

fn build_layout(flex: &mut stretch::node::Stretch, node: &mut Node) -> Result<()> {
    use stretch::geometry;

    let flex_style = node.as_computed_style().flex_style;
    match node.to_mut_children() {
        Some(nodes) => {
            let mut children = vec![];
            for node in nodes.iter_mut() {
                build_layout(flex, node)?;
                children.push(node.to_flex_node());
            }
            let flex_node = err_at!(Invalid, flex.new_node(flex_style, children))?;
            node.set_flex_node(flex_node);
        }
        None => {
            let size: geometry::Size<f32> = node.to_extent().into();
            node.set_flex_node(err_at!(
                Invalid,
                flex.new_leaf(
                    flex_style,
                    Box::new(move |x| {
                        trace!("{:?}, {:?}", x, size);
                        Ok(size)
                    })
                )
            )?);
        }
    }

    Ok(())
}

fn gather_layout(flex: &stretch::node::Stretch, node: &mut Node) -> Result<()> {
    node.set_box_layout(
        err_at!(Invalid, flex.layout(node.to_flex_node()))?
            .clone()
            .into(),
    );
    if let Some(nodes) = node.to_mut_children() {
        for node in nodes.iter_mut() {
            gather_layout(flex, node)?
        }
    }

    Ok(())
}
