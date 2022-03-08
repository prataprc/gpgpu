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
