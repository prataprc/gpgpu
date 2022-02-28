* comments
* tokens
  * a literal
    * a numeric literal - `int_literal`, `uint_literal`, `float_literal`, `float_literal`, `decimal_float_literal`, `hex_float_literal`
      * what is the difference between `-5` and `- 5`
    * a boolean literal: either true or false.
  * a keyword
    * `bool`, `f32`, `i32`, `u32`, `false`, `true`
    * `vec2<i32>`, `vec3<i32>`, `vec4<i32>`, `vec2<f32>`, `vec3<f32>`, `vec4<f32>`,
    * `mat2x2`, `mat2x3`, `mat2x4`,
    * `mat3x2`, `mat3x3`, `mat3x4`,
    * `mat4x2`, `mat4x3`, `mat4x4`,
    * `texture_1d`, `texture_2d`,  `texture_3d`,
    * `texture_storage_1d`, `texture_storage_2d`, `texture_storage_3d`,
    * `texture_2d_array`, `texture_storage_2d_array`, `texture_cube_array`, `texture_depth_2d_array`, `texture_depth_cube_array`,
    * `texture_cube`, `texture_multisampled_2d`,
    * `texture_depth_2d`, `texture_depth_cube`, `texture_depth_multisampled_2d`,
    * `fn`, `function`, `let`, `var`, `return`
    * `switch` `if`, `else`, `for` `loop`, `while`
    * `break`, `case`, `continue` `default`,
    * `array`
    * `atomic`
    * `override`
    * `ptr`
    * `sampler`
    * `sampler_comparison`
    * `struct`
    * `discard`
    * `enable`
    * `fallthrough`
    * `private`
    * `storage`
    * `type`
    * `uniform`
    * `workgroup`
    * `bitcast`
    * `continuing`
  * a reserved word
  * a syntactic token
  * an identifier
    * a type
    * a value
    * a variable
    * a function
    * a formal parameter
    * must not have the same spelling as a keyword or as a reserved word.
    * must not be _ (a single underscore)
    * must not start with two underscores
* blankspace

Example of comments:

```wgsl
let f = 1.5;   // This is line-ending comment.
let g = 2.5;   /* This is a block comment
                  that spans lines.
                  /* Block comments can nest.
                   */
                  But all block comments must terminate.
                */
```

Note: The return type for some built-in functions are structure types whose name cannot
be used WGSL source. Those structure types are described as if they were predeclared with
a name starting with two underscores. The result value can be saved into newly declared
let or var using type inferencing, or immediately have one of its members immediately
extracted by name. See example usages in the description of `frexp` and `modf`.

* Attributes
  * An attribute modifies an object or type.
  * An attribute must not be specified more than once per object or type.

* Resource, bind-group.

* type-checking, expression, sub-expression, static-context, dynamic-context
* plain-types - scalar-type, atomic-type, composite-type
  * scalar-type `u32, i32, f32, bool`
  * atomic-type `atomic<T>`, where N is i32 or u32.
  * vector-type `vecN<T>,` where N is {2,3,4} and T is scalar-type.
  * matrix type `matNxM<f32>` where N is {2,3,4} columns and M is {2,3,4} rows.
  * array-type, `array<E,N>` or `array<E>`, where E is type of element and N is count.
    * Element type must be either scalar, vector, matrix, atomic, array or struct type.
    * The element count value is fully determined at pipeline creation time.
  * structure-type members can be of,
    * scalar, vector, matrix, atomic, fixed-size-array, struct (fixed-size)
    * a runtime-sized array type, but only if it is the last member of the structure
  * composite-types - vector, matrix, array, struct
* memory-scope: workgroup-memory-scope, queue-family-memory-scope.
