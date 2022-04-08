vertex          color               waves
edges           brightness          oscillation
primitives      depth               harmonics
modelling       transparency        frequency
geometry        opacity             amplitude
                diffuse             photon-absorb
                irridescence        photon-reflect
                conductors          photon-transmit
                dielectrics         photon-energy

ray-geometry-intersection
compositing

aliasing and anti-aliasing
super-sampling
multi-sampling
coverage
bit-blitting function
alpha-blending function
centroid-interpolation-qualifier function

image-format - color,depth,stencil,depth/stencil
  color - float, signed-normalised-int, unsigned-normalised-int, int, uint
  depth - float, unsigned-normalised-int, signed-normalised-int
  stencil - uint
texture-coordinates
autofill for RGBA, RGB to 0 and A to 1.0
sampler
image-load-store


geometry
--------

points, vectors, normals.
curve, tangent-line. surface, tangent-plane.
normalization of vectors (TBD)
linear operations on points in a coordinate.
  transformation, change of basis, change of coordinates
transformation
  translate
  scale
  rotate
In linear algebra,
  1 or 2 or 3 axes form the "basis" of a coordinate system.
  a basis is set of linearly independent vectors.
  in a linear combination (aka span) can represent a plan (2-axes) or space (3-axes)
psuedo-vectors and surface-normals.


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
