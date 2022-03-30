* Shader stages : compute, vertex, fragment

#### Shader lifec-cycle

* Shader module creation
  * This occurs when the `createShaderModule` method is called. The source text
    for a WGSL program is provided at this time.
* Pipeline creation
  * This occurs when the `createComputePipeline` method or the `createRenderPipeline`
    method is invoked. These methods use one or more previously created shader modules,
    together with other configuration information.
* Shader execution start
  * This occurs when a draw or dispatch command is issued to the GPU, begins executing
    the pipeline, and invokes the shader stage entry point function.
* Shader execution end. This occurs when all work in the shader completes:
  * all its invocations terminate
  * and all accesses to resources complete
  * outputs, if any, are passed to downstream pipeline stages.


#### Builtin @stage(vertex) input values

* `vertex_index`, u32
   * Index of the current vertex within the current API-level draw command, independent of draw instancing.
   * For a non-indexed draw, the first vertex has an index equal to the firstVertex argument of the draw, whether provided
     directly or indirectly. The index is incremented by one for each additional vertex in the draw instance.
   * For an indexed draw, the index is equal to the index buffer entry for vertex, plus the baseVertex argument of the draw,
     whether provided directly or indirectly.
* `instance_index`, u32
  * Instance index of the current vertex within the current API-level draw command.
  * The first instance has an index equal to the firstInstance argument of the draw, whether provided directly or indirectly.
    The index is incremented by one for each additional instance in the draw.

#### Builtin @stage(vertex) output values

* `position`, vec4<f32>
  * Output position of the current vertex, using homogeneous coordinates. After homogeneous normalization (where each
    of the x, y, and z components are divided by the w component), the position is in the WebGPU normalized device coordinate space.

#### Builtin @stage(fragment) input values

* `position`, vec4<f32>
  Framebuffer position of the current fragment, using normalized homogeneous coordinates. (The x, y, and z components have
  already been scaled such that w is now 1.)
* `front_facing`, bool
  * True when the current fragment is on a front-facing primitive. False otherwise.
* `sample_mask`, u32
  * Sample coverage mask for the current fragment. It contains a bitmask indicating which samples in this fragment are covered
    by the primitive being rendered.

#### Builtin @stage(fragment) output values

* `frag_depth`, f32
  * Updated depth of the fragment, in the viewport depth range.
* `sample_index`, u32
  * Sample index for the current fragment. The value is least 0 and at most sampleCount-1, where sampleCount is the number
    of MSAA samples specified for the GPU render pipeline.
* `sample_mask`, u32
  * Sample coverage mask control for the current fragment. The last value written to this variable becomes the shader-output
    mask. Zero bits in the written value will cause corresponding samples in the color attachments to be discarded.

#### Builtin @stage(fragment) input values

* `local_invocation_id`, vec3<u32>
  * The current invocation’s local invocation ID, i.e. its position in the workgroup grid.
* `local_invocation_index`, u32
  * The current invocation’s local invocation index, a linearized index of the invocation’s position within the workgroup grid.
* `global_invocation_id`, vec3<u32>
  * The current invocation’s global invocation ID, i.e. its position in the compute shader grid.
* `workgroup_id`, vec3<u32>
  * The current invocation’s workgroup ID, i.e. the position of the workgroup in the workgroup grid.
* `num_workgroups`, vec3<u32> 
  * The dispatch size, vec<u32>(`group_count_x`, `group_count_y`, `group_count_z`), of the compute shader dispatched by the API.

