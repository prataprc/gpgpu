Simple types
------------

VkDeviceType, VkMemoryType, VkMemoryHeap

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


VkSurfaceKHR
  VkSurfaceCapabilities {
     minImageCount, maxImageCount,
     currImageExtent, minImageExtent, maxImageExtent, maxImageArrayLayers,
     VkSurfaceTransformFlagsKHR, VkCompositeAlphaFlagsKHR, VkImageUsageFlags
  }
  VkSurfaceFormat { VkFormat, VkColorSpace }
