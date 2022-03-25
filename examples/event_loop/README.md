Start an event loop and an associated window. This example program is a good starting point
for acclamatising with [winit][winit] package, especially to understand the different events
it can generate for the application and [window attributes][window-attribute].

#### Event Logging

To trace all events generated for the application:

```bash
RUST_LOG=trace WINIT_UNIX_BACKEND=x11 cargo run --example event_loop
```

note that `RUST_LOG` environment variable is set to `debug`. Trace logging can generate
a large dump of events like `Event::RedrawEventsCleared`, `Event::MainEventsCleared`. To
filter out frequently generated events and only log user inputs, set the `RUST_LOG`
variable to `trace`:

```bash
RUST_LOG=debug WINIT_UNIX_BACKEND=x11 cargo run --example event_loop
```

#### Window configuration

[winit][winit] allows several user defined attributes while instantiating a window. Refer
to [example toml file][../../config.toml] that can be used to configure window attributes.

```bash
WINIT_UNIX_BACKEND=x11 cargo run --example event_loop -- --config config.toml
```


winit: https://docs.rs/winit/latest/winit/
window-attribute: https://docs.rs/winit/latest/winit/window/struct.WindowAttributes.html
