```
                            Backends
                                |
                                V        create_surface(W)
                            Instance ------------------------> Surface
                                |                               |
                      request_adapter(options)                  | get_preferred_format(Adapter)                   describe()
                                |                               *--------------------------------> TextureFormat ------------> TextureFormatInfo
                                V                               | get_current_texture()
                             Adapter                            *--------------------------------> SurfaceTexture { Texture, .. }
                                |                                                                        |
  is_surface_supported(Surface) |                                                                        | present()
                                |                                                                        *-----------*
                     features() |
          Features <------------|
                        limit() |
             Limit <------------|
                     get_info() |
              Info <------------|
  get_texture_format_features() |
TextureFormatFeatures <---------|
                                *
```

```
                 projection                       perspective
                 transformation                   division
view-coordinate ---------------> clip-coordinate -------------> NDC

```

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
