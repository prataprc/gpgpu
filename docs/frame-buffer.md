* frame-buffer = width * height * `color_depth`
* monochrome(1), palettized(4/8), high-color(16), true-color(24bits), alpha-channel
* modes - resolution, color-depth, memory-layout, refresh-rate.
* access - memory-mapping, port-commands, bank-switching
* memory layout - packed-pixel, planar.
* packed pixed - pixel bits are consecutively arranged.
* planar - one place for each bit, parallel access to plane possible.

* page flipping, aka double buffering; vertical-banking-interval VBLANK, VBI.
* bit blitting; <source> OP [mask] OP [stencil] OP <dest> upto 4 bitmaps, OP is boolean
* alpha compositing; successor to bit-blitting

* virtual framebuffers - fbdev (linux frame-buffer), xvfb (X frame-buffer).

[1]: https://en.wikipedia.org/wiki/Framebuffer
[2]: https://linux.die.net/man/5/fb.modes
[3]: https://en.wikipedia.org/wiki/Bank_switching
[4]: https://en.wikipedia.org/wiki/Packed_pixel
[5]: https://en.wikipedia.org/wiki/Planar_(computer_graphics)

