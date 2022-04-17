vertex          color               waves               foreshortening              model-space             world-matrix
edges           brightness          oscillation         sterioscoping-vision        world-space             view-matrix
primitives      depth               harmonics           origin                      camera/view-space       projection-matrix
modelling       transparency        frequency           vertices                    clip/projection-space   view-port
geometry        opacity             amplitude           topology-model              screen-space
camera          diffuse             photon-absorb       viewing-frustum
lights          irridescence        photon-reflect      perspective-projection
                conductors          photon-transmit     scene
                dielectrics         photon-energy


pitch-(x)       aliasing and anti-aliasing                      ray-geometry-intersection
yaw---(y)       super-sampling                                  compositing
roll--(z)       multi-sampling
                coverage
                bit-blitting function
                alpha-blending function
                centroid-interpolation-qualifier function

smooth-shading
sampling
tesselation
polygonal-mesh
NURBS
surface-subdivision
solid-body-simulation


image-format - color,depth,stencil,depth/stencil
  color - float, signed-normalised-int, unsigned-normalised-int, int, uint
  depth - float, unsigned-normalised-int, signed-normalised-int
  stencil - uint
texture-coordinates
autofill for RGBA, RGB to 0 and A to 1.0
sampler
image-load-store


GPU
---

* vertice-op -> primitive-assembly -> rasterization - fragment-op - composition
  * and, one or more light-source.
  * and, viewer's position and orientation.

* hardware spritting (TBD)
* scaneline rendering (TBD)
* tiled rendering (TBD)
* forward rendering (TBD)
* deferred rendering (TBD)
* feed-forward graphics pipeline (TBD)
* SPMD - Single-program-multiple-data
* SPIRV - Intermediate IR for GPU.
* GLSL - C like Graphics Library Shader Language
