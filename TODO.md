* Cleanup [dependencies] in Cargo.toml
* Assert `ttf_parser::Rect` min and max attributes for x and y axis.
  Doesn’t guarantee that `x_min <= x_max` and/or `y_min <= y_max`.
* How to figure out which winit::VideoMode is used by winit::MonitorHandle
