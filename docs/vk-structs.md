Simple types
------------

VkDeviceType, VkMemoryType, VkMemoryHeap, VkExtent2D, VkOffset2D,
VkRect2D { VkOffset2D, VkExtent2D }

Dispatchable Handles
--------------------

VkInstance, VkPhysicalDevice, VkDevice, VkQueue, VkCommandBuffer

Non-Dispatchable Handles
------------------------

VkDeviceMemory, VkSurfaceKHR, VkSwapchainKHR, VkDisplayKHR, VkDisplayModeKHR
VkCommandPool
VkQueryPool
VkBuffer, VkBufferView, VkImage, VkImageView, VkSampler, VkSamplerYcbcrConversion
VkShaderModule
VkPipelineLayout, VkPipeline, VkPipelineCache
VkEvent, VkSemaphore, VkFence
VkDescriptorSetLayout, VkDescriptorPool, VkDescriptorSet, VkDescriptorUpdateTemplate
VkRenderPass, VkFramebuffer

Other structs
-------------

Structs
-------

VkInstance
  fnCreateInstance, fnDestroyInstance
  fnGetInstanceProcAddr
  fnEnumeratePhysicalDevices
VkPhysicalDevice
  fnEnumerateDeviceExtensionProperties -> VkExtensionProperties
  fnEnumerateDeviceLayerProperties -> VkLayerProperties
  fnGetPhysicalDeviceFeatures -> VkPhysicalDeviceFeatures
  fnGetPhysicalDeviceProperties -> VkPhysicalDeviceProperties {
     VkPhysicalDeviceType, VkPhysicalDeviceLimits, VkPhysicalDeviceSparseProperties
  }
  fnGetPhysicalDeviceMemoryProperties -> VkPhysicalDeviceMemoryProperties { VkMemoryType, VkMemoryHeap }
  fnGetPhysicalDeviceQueueFamilyProperties -> VkQueueFamilyProperties { VkQueueFlagBits }
  VkFormat
    fnGetPhysicalDeviceFormatProperties -> VkFormatProperties
  VkFormat, VkImageType, VkImageTiling, VkImageCreateFlagBits, VkImageUsageFlagBits
    fnGetPhysicalDeviceImageFormatProperties -> VkImageFormatProperties
  VkFormat, VkImageType, VkImageTiling, VkSampleCountFlagBits, VkImageUsageFlagBits
    fnGetPhysicalDeviceSparseImageFormatProperties -> VkSparseImageFormatProperties
  VkPhysicalDeviceExternalBufferInfo { VkBufferCreateFlagBits, VkBufferUsageFlagBits, VkExternalMemoryHandleTypeFlagBits }
    fnGetPhysicalDeviceExternalBufferProperties -> VkExternalBufferProperties {
        VkExternalMemoryProperties { VkExternalMemoryFeatureFlagBits,  VkExternalMemoryHandleTypeFlagBits
    }
  VkPhysicalDeviceExternalFenceInfo { VkExternalFenceHandleTypeFlagBits }
    fnGetPhysicalDeviceExternalFenceProperties -> VkExternalFenceProperties {
        VkExternalFenceFeatureFlagBits VkExternalFenceHandleTypeFlagBits,
    }
  VkPhysicalDeviceExternalSemaphoreInfo { VkExternalSemaphoreHandleTypeFlagBits }
    fnGetPhysicalDeviceExternalSemaphoreProperties -> VkExternalSemaphoreProperties {
        VkExternalSemaphoreFeatureFlagBits, VkExternalSemaphoreHandleTypeFlagBits
    }
  VkDeviceCreateInfo {
        VkDeviceQueueCreateInfo { flags, queueFamilyIndex, queueCount, pQueuePriorities },
        layers, extensions, VkPhysicalDeviceFeatures
    }
    fnCreateDevice -> VkDevice
  fnDestroyDevice
VkDevice
  fnGetDeviceProcAddr,
  fnGetDeviceQueue,
  fnAllocateMemory, fnFreeMemory,
  fnGetDeviceMemoryCommitment, fnGetDeviceMemoryOpaqueCaptureAddress,
  fnDeviceWaitIdle,
  fnMapMemory, fnUnmapMemory,
  fnFlushMappedMemoryRanges,
  fnInvalidateMappedMemoryRanges,

  fnCreateCommandPool, fnAllocateCommandBuffers
  fnResetCommandPool, fnTrimCommandPool,
  fnDestroyCommandPool,fnFreeCommandBuffers,

  fnCreateDescriptorPool, fnCreateDescriptorSetLayout, fnCreateDescriptorUpdateTemplate, fnAllocateDescriptorSets
  fnGetDescriptorSetLayoutSupport, fnUpdateDescriptorSetWithTemplate, fnUpdateDescriptorSets, fnResetDescriptorPool,
  fnDestroyDescriptorPool, fnDestroyDescriptorSetLayout, fnDestroyDescriptorUpdateTemplate, fnFreeDescriptorSets,

  fnCreateBuffer, fnBindBufferMemory, fnCreateBufferView,
  fnGetBufferDeviceAddress, fnGetBufferMemoryRequirements, fnGetBufferOpaqueCaptureAddress,
  fnDestroyBuffer, fnDestroyBufferView,

  fnCreateImage, fnBindImageMemory, fnCreateImageView,
  fnGetImageMemoryRequirements, fnGetImageSparseMemoryRequirements, fnGetImageSubresourceLayout,
  fnDestroyImage, fnDestroyImageView, 

  fnCreateComputePipelines, fnCreateGraphicsPipelines, fnCreatePipelineLayout, fnCreatePipelineCache,
  fnGetPipelineCacheData, fnMergePipelineCaches,
  fnDestroyPipeline, fnDestroyPipelineCache, fnDestroyPipelineLayout,
  fnCreateSampler, fnCreateSamplerYcbcrConversion, fnCreateShaderModule,
  fnDestroySampler, fnDestroySamplerYcbcrConversion, fnDestroyShaderModule,
  fnCreateQueryPool, fnGetQueryPoolResults, fnResetQueryPool, fnDestroyQueryPool,

  fnCreateEvent, fnCreateFence, fnCreateSemaphore, 
  fnGetEventStatus, fnGetFenceStatus, fnGetSemaphoreCounterValue, fnSetEvent, fnResetEvent, fnResetFences,
  fnSignalSemaphore, fnWaitForFences, fnWaitSemaphores
  fnDestroyEvent, fnDestroyFence, fnDestroySemaphore,

  fnCreateFramebuffer, fnCreateRenderPass,
  fnGetRenderAreaGranularity,
  fnDestroyFramebuffer, fnDestroyRenderPass,
VkQueue
VkCommandBuffer


VkDiskplayMode {
  VkExtent2D, refreshRate
}
VkDisplaySurfaceCreateInfo {
  VkDisplayMode, planeIndex, planeStackIndex, VkSurfaceTransformFlags,
  globalAlpha, alphaMode, VkExtent2D
} -> VkSurface
VkDisplayProperties {
  name, VkExtent2D, VkExtent2D, VkSurfaceTransformFlags, planeReorderPossible,
  persistentContent
}
VkDisplayPlaneAlphaFlagBits {
  `VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR`, `VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR`
  `VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR`
  `VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR`
}
VkDisplayPlaneCapabilities
VkDisplayPresentInfo { VkRect2D, VkRect2D, persistent }

VkSurfaceKHR
  VkSurfaceFormat { VkFormat, VkColorSpace }
  VkSurfaceCapabilities {
     minImageCount, maxImageCount,
     currImageExtent, minImageExtent, maxImageExtent, maxImageArrayLayers,
     VkSurfaceTransformFlags, VkCompositeAlphaFlags, VkImageUsageFlags
  }
  VkCompositeAlphaFlagBits {
    VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 0x00000001,
    VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 0x00000002,
    VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 0x00000004,
    VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 0x00000008,
  }
  VkPresentModeKHR {
    VK_PRESENT_MODE_IMMEDIATE_KHR = 0,
    VK_PRESENT_MODE_MAILBOX_KHR = 1,
    VK_PRESENT_MODE_FIFO_KHR = 2,
    VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3,
    // Provided by VK_KHR_shared_presentable_image
    VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR = 1000111000,
    // Provided by VK_KHR_shared_presentable_image
    VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR = 1000111001,
  }

VkSwapChain
  VkSharingMode {
    VK_SHARING_MODE_EXCLUSIVE = 0,
    VK_SHARING_MODE_CONCURRENT = 1,
  }
  VkSwapchainCreateFlagBitsKHR {
    // Provided by VK_KHR_swapchain with VK_VERSION_1_1, VK_KHR_device_group with VK_KHR_swapchain
    VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR,
    // Provided by VK_KHR_swapchain with VK_VERSION_1_1
    VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR,
    // Provided by VK_KHR_swapchain_mutable_format
    VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR,
  }
  VkSwapchainCreateInfoKHR {
    VkSurface, VkSwapchainCreateFlagBitsKHR, minImageCount, VkFormat, VkColorSpace,
    VkExtent2D, imageArrayLayers, VkImageUsageFlagBits, VkSharingMode, QueueFamilies
    VkSurfaceTransformFlag, VkCompositeAlphaFlag, VkPresentMode, clipped, oldSwapchain
  }
