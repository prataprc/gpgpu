/// Trait that needs to be implemented by types abstracting an OS window and optionally
/// an event loop.
pub trait AppWindow<W>
where
    W: Windowing,
{
    /// Return reference to underlying window instance.
    fn as_window(&self) -> &W
    where
        W: Windowing;
}

/// Trait that needs to be implemented by types returned by [AppWindow::as_window] trait
/// method.
pub trait Windowing {
    /// Return the size of window's client area as (width, height).
    fn inner_size(&self) -> (u32, u32);
}

impl Windowing for winit::window::Window {
    fn inner_size(&self) -> (u32, u32) {
        let size = self.inner_size();
        (size.width, size.height)
    }
}
