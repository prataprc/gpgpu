Data types
----------

basic type and aggreegate types.
scalar types and vector types.
  scalar - bool, int, uint, float, double
  vector - bvec2, bvec3, bvec4
         - ivec2, ivec3, ivec4, uvec2, uvec3, uvec4
         - vec2, vec3, vec4, dvec2, dvec3, dvec4
  swizzling
  matrix are float or double.
    - mat2, mat23, mat24,
    - mat32, mat3, mat34,
    - mat42, mat34, mat4,
    - dmat2, dmat23, dmat24,
    - dmat32, dmat3, dmat34,
    - dmat42, dmat34, dmat4,
opaque types
  used as uniforms or function input vars
  sampler type - floating-point, signed-integer, unsigned-integer
     - gsampler1D, gsampler2D, gsampler3D, gsamplerCube,
     - gsampler2DRec, gsampler1DArray, gsampler2DArray,
     - gsamplerCubeArray, gsamplerBuffer, gsampler2DMS,
     - gsampler2DMSArray
  image type
     - gimage1D, gimage2D, gimage3D, gimageCube,
     - gimage2DRec, gimage1DArray, gimage2DArray,
     - gimageCubeArray, gimageBuffer, gimage2DMS,
     - gimage2DMSArray
  atomic type
    `atomic_uint`
aggregate types
  array - homogenous sequences of basic-types

implicit conversion
  - signed integers to unsigned integers, but not vice-versa.
  - either integer type can be converted into floats.
  - integers and floats can be converted into doubles.
  - vector and matrix values if basic type they have is convertible.

uninitialized variables
  - input or output qualified variables
  - any variable of a opaque type
  - variables declared in interface-block

literals
  - always basic types

Scope
-----

qualifiers
  default, constant, in,

global variables - default, constant
local variables - default, constant
function parameters - constant
shader stage - in, out
uniform
interface block definitions
function return values

in/out
buffer-backed interface block
shader storage interface block
qualifiers
constant expressions


* C/C++ like syntax.
* #define #undef
  #if #ifdef #ifndef #elfi #else #endif 
  #error #pragma #extension #version #line
* macros begining with `__` and `GL_` are reserved.
* `__LINE__`, `__VERSION__`, `__FILE__` are reserved.

* `#pragma optimize(on)`
* `#pragma optimize(off)`
* `#pragma debug(on)`
* `#pragma debug(off)`
* `#version number [core | compatibility | es]`
