* Physical-Device -> Queues -> Queue-Family (graphics, compute, transfer, sparse).
* Memory properties (all memory visible to device)
  * Device local.
  * Device local, host visible (coherent, cached).
  * Host local, host visibile.
* Memory organised as images and buffers.

* Execution between host and device is async.
* Execution between queues within device is async.
* Synchronization between host and device and between queues, application responsibility

Image attributes
----------------

(VkFormat)

UNDEFINED = 0,
R G B A D16 D24 D32 S8 X8
UNORM SNORM USCALED SSCALED UINT SINT UFLOAT SFLOAT
PACK8 PACK16 PACK32 BLOCK
SRGB
BC1 BC2 BC3 BC4 BC5 BC6H BC7 ETC2 EAC ASTC

(VkImageType)

`VK_IMAGE_TYPE_1D`, `VK_IMAGE_TYPE_2D`, `VK_IMAGE_TYPE_3D`

(VkImageTiling)

`VK_IMAGE_TILING_OPTIMAL`, `VK_IMAGE_TILING_LINEAR`, `_DRM_FORMAT_MODIFIER_EXT`

(VkImageUsage)

`VK_IMAGE_USAGE_TRANSFER_SRC_BIT`
`_TRANSFER_DST_BIT`
`_USAGE_SAMPLED_BIT`
`_USAGE_STORAGE_BIT`
`_USAGE_COLOR_ATTACHMENT_BIT`
`_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT`
`_USAGE_TRANSIENT_ATTACHMENT_BIT`
`_USAGE_INPUT_ATTACHMENT_BIT`
`_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT`
`VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`

Resource/Descriptor type binding
--------------------------------

Sampler
  An object that contains state that controls how sampled image data is sampled
  (or filtered) when accessed in a shader. Also a descriptor type describing the
  object. Represented by a VkSampler object.

  `VK_DESCRIPTOR_TYPE_SAMPLER` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`

Sampled Image
  A descriptor type that represents an image view, and supports filtered (sampled)
  and unfiltered read-only acccess in a shader.

  `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` or `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`

Combined Image Sampler
  A descriptor type that includes both a sampled image and a sampler.

  `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`

Storage Image
  A descriptor type that represents an image view, and supports unfiltered loads,
  stores, and atomics in a shader.

  `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`

Uniform Texel Buffer
  A descriptor type that represents a buffer view, and supports unfiltered, formatted,
  read-only access in a shader.

  `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`

Storage Texel Buffer
  A descriptor type that represents a buffer view, and supports unfiltered, formatted
  reads, writes, and atomics in a shader.

  `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`

Uniform Buffer
  A descriptor type that represents a buffer, and supports read-only access in a shader.

  `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`

Storage Buffer
  A descriptor type that represents a buffer, and supports reads, writes, and atomics in
  a shader.

  `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`

Input Attachment
  A descriptor type that represents an image view, and supports unfiltered
  read-only access in a shader, only at the fragment’s location in the view.

  `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT`

Dynamic Uniform Buffer
  A uniform buffer whose offset is specified each time the uniform buffer is bound to a
  command buffer via a descriptor set.

Dynamic Storage Buffer
  A storage buffer whose offset is specified each time the storage buffer is bound to a
  command buffer via a descriptor set.

inline uniform block
  `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT`

acceleration structure
  `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
  ` VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV`

Render pass
-----------

VkMemory
  is just a sequence of N bytes in memory.
VkImage
  object adds to it e.g. information about the format
  (so you can address by texels, not bytes).
VkImageView
  object helps select only part (array or mip) of the VkImage
  (like stringView, arrayView or whathaveyou does).
  Also can help to match to some incompatible interface (by type casting format).
VkFramebuffer
  binds a VkImageView with an attachment.
VkRenderpass
  defines which attachment will be drawn into


Objects
-------

* There is no global state in vulkan.
* Instance objects.
* Device objects.
* Dispatchable objects.

* VkBool32, `VK_TRUE`, `VK_FALSE`.
* VkDeviceSize, VkDeviceAddress, `Vk*Flags`.
* VkCreateInfo

VkInstance
  -> VkPhysicalDevice
  -> VkDevice,
     -> VkQueue,
     -> VkCommandBuffer,

  VkDeviceMemory, VkPipeline, VkCommandBuffer,

  VkSampler, VkFramebuffer, VkRenderPass
  VkQueryPool, VkDescriptorPool, VkCommandPool, VkDescriptorSet,

  VkShaderModule, VkPipelineCache, VkRenderPass, VkPipelineLayout,
  VkDescriptorSetLayout
  VkEvent, VkBuffer, VkBufferView, VkImage, VkImageView,
  VkSampleYcbcrConversion,

Execution
---------

* Function entry points `InstanceProcAddr`
  - vkGetInstanceProcAddr
  - vkEnumerateInstanceVersion
  - vkEnumerateInstanceExtensionProperties
  - vkEnumerateInstanceLayerProperties
  - vkCreateInstance
  - <core-command>
  - <instance-extension-command>
  - <device-extension-command>
* Function entry points `DeviceProcAddr`
  - <core-command>
  - <device-extension-command>

Commands
--------

* Core-Vulkan commands.
* Extension commands (Instance-wise, device-wise, layer-wise).

* Bind with pipeline.
* Descriptor sets.
* Modify dynamic state.
* Draw.
* Dispatch.
* Execute secondary command buffers.
* Copy Buffers and Images.

* Command buffer, to record commands into. Submit for execution.
* Batch execution of command-buffer.
* Command buffers contains device commands - immutable and reusable.
* Two levels of command buffer, primary command buffers, which can execute secondary
  command buffers, and which are submitted to queues, and secondary command buffers,
  which can be executed by primary command buffers, and which are not directly
  submitted to queues.

VkCommandBuffer

* Type of commands -
  * To bind to pipelines.
  * 

* Work submission commands to Queue -
  vkQueueSubmit, vkQueueBindSparse
* Semaphore Wait Commands -
* Sempahore Done Commands -

Pipeline
--------

* Graphics-pipeline or Compute-pipeline.
* Geometric primitives - points, lines and triangles. ???what is Primitive topology
* Vertex described by (position and attributes) ???what are the attributes

Synchronisation
---------------

* Submission order.
* Implicit ordering guarantees.

Sparse memory Binding
---------------------
