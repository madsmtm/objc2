data! {
    // TODO: Verify that index out-of-bounds is sound for various arrays.
    // class MyArray {
    //     unsafe -objectAtIndexedSubscript;
    //     unsafe -setObject_atIndexedSubscript;
    // }

    class MTLPrimitiveAccelerationStructureDescriptor {
        unsafe +descriptor;
        unsafe -setGeometryDescriptors;
    }

    class MTLAccelerationStructureGeometryDescriptor {
        unsafe -setPrimitiveDataBuffer;
        unsafe -setPrimitiveDataStride;
        unsafe -setPrimitiveDataElementSize;
        unsafe -setIntersectionFunctionTableOffset;
    }

    class MTLAccelerationStructureTriangleGeometryDescriptor {
        unsafe +descriptor;
        unsafe -setIndexBuffer;
        // unsafe -setIndexType;
        unsafe -setVertexBuffer;
        unsafe -setVertexStride;
        unsafe -setTriangleCount;
    }

    class MTLAccelerationStructureBoundingBoxGeometryDescriptor {
        unsafe +descriptor;
        unsafe -setBoundingBoxBuffer;
        unsafe -setBoundingBoxCount;
    }

    class MTLInstanceAccelerationStructureDescriptor {
        unsafe +descriptor;
        unsafe -setInstancedAccelerationStructures;
        unsafe -setInstanceCount;
        unsafe -setInstanceDescriptorBuffer;
    }

    protocol MTLAccelerationStructureCommandEncoder {
        unsafe -buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset;
        unsafe -writeCompactedAccelerationStructureSize_toBuffer_offset;
        unsafe -copyAndCompactAccelerationStructure_toAccelerationStructure;
    }

    class MTLIntersectionFunctionTableDescriptor {
        unsafe +new;
        unsafe -setFunctionCount;
    }

    protocol MTLIntersectionFunctionTable {
        unsafe -setFunction_atIndex;
    }

    class MTLStructMember {
        unsafe -name;
        unsafe -offset;
        unsafe -dataType;
        unsafe -structType;
        unsafe -arrayType;
    }

    class MTLStructType {
        unsafe -members;
        unsafe -memberByName;
    }

    class MTLArrayType {
        unsafe -elementType;
        unsafe -arrayLength;
        unsafe -stride;
        unsafe -elementStructType;
        unsafe -elementArrayType;
    }

    class MTLArgument {
        unsafe -name;
        unsafe -type_;
        unsafe -access;
        unsafe -index;
        unsafe -isActive;
        unsafe -bufferAlignment;
        unsafe -bufferDataSize;
        unsafe -bufferDataType;
        unsafe -bufferStructType;
        unsafe -threadgroupMemoryAlignment;
        unsafe -threadgroupMemoryDataSize;
        unsafe -textureType;
        unsafe -textureDataType;
    }

    class MTLArgumentDescriptor {
        unsafe +argumentDescriptor;
        unsafe -setDataType;
        unsafe -setIndex;
        unsafe -setAccess;
        // unsafe -setArrayLength;
        unsafe -setTextureType;
    }

    protocol MTLBuffer {
        unsafe -length;
        unsafe -contents;
        unsafe -didModifyRange;
        unsafe -newTextureWithDescriptor_offset_bytesPerRow;
        unsafe -addDebugMarker_range;
        unsafe -removeAllDebugMarkers;
        unsafe -remoteStorageBuffer;
        unsafe -newRemoteBufferViewForDevice;
        unsafe -gpuAddress;
    }

    class MTLCaptureDescriptor {
        unsafe +new;
        unsafe -destination;
        unsafe -setDestination;
        unsafe -outputURL;
        unsafe -setOutputURL;
    }

    protocol MTLCaptureScope {
        unsafe -beginScope;
        unsafe -endScope;
        unsafe -label;
    }

    // Note: MTLCaptureManager is not documented thread-safe, so
    // +sharedCaptureManager is not safe either, since we do interior mutation
    // here.
    class MTLCaptureManager {
        unsafe -newCaptureScopeWithDevice;
        unsafe -newCaptureScopeWithCommandQueue;
        unsafe -supportsDestination;

        unsafe -startCaptureWithDescriptor_error;
        unsafe -startCaptureWithDevice;
        unsafe -startCaptureWithCommandQueue;
        unsafe -startCaptureWithScope;
        unsafe -stopCapture;

        unsafe -defaultCaptureScope;
        unsafe -setDefaultCaptureScope;

        unsafe -isCapturing;
    }

    protocol MTLCommandBuffer {
        unsafe -label;
        unsafe -setLabel;
        unsafe -enqueue;
        unsafe -commit;
        unsafe -presentDrawable;
        unsafe -waitUntilScheduled;
        // TODO once blocks are better
        // unsafe -addCompletedHandler;
        unsafe -status;
        unsafe -blitCommandEncoder;
        unsafe -renderCommandEncoderWithDescriptor;
        unsafe -computeCommandEncoder;
        unsafe -computeCommandEncoderWithDispatchType;
        unsafe -encodeWaitForEvent_value;
        unsafe -encodeSignalEvent_value;
        unsafe -parallelRenderCommandEncoderWithDescriptor;
        unsafe -accelerationStructureCommandEncoder;
        unsafe -pushDebugGroup;
        unsafe -popDebugGroup;
    }

    protocol MTLCommandQueue {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -commandBuffer;
    }

    class MTLStencilDescriptor {
        unsafe -stencilCompareFunction;
        unsafe -setStencilCompareFunction;
        unsafe -stencilFailureOperation;
        unsafe -setStencilFailureOperation;
        unsafe -depthFailureOperation;
        unsafe -setDepthFailureOperation;
        unsafe -depthStencilPassOperation;
        unsafe -setDepthStencilPassOperation;
        unsafe -readMask;
        unsafe -setReadMask;
        unsafe -writeMask;
        unsafe -setWriteMask;
    }

    class MTLDepthStencilDescriptor {
        unsafe -depthCompareFunction;
        unsafe -setDepthCompareFunction;
        unsafe -isDepthWriteEnabled;
        unsafe -setDepthWriteEnabled;
        unsafe -frontFaceStencil;
        unsafe -setFrontFaceStencil;
        unsafe -backFaceStencil;
        unsafe -setBackFaceStencil;
        unsafe -label;
        unsafe -setLabel;
    }

    protocol MTLDepthStencilState {
        unsafe -label;
        unsafe -device;
    }

    // TODO:
    // unsafe fn MTLCreateSystemDefaultDevice;
    // unsafe fn MTLCopyAllDevices;

    protocol MTLDevice {
        unsafe -name;
        unsafe -registryID;
        unsafe -maxThreadsPerThreadgroup;
        unsafe -isLowPower;
        unsafe -isHeadless;
        unsafe -isRemovable;
        unsafe -hasUnifiedMemory;
        unsafe -recommendedMaxWorkingSetSize;
        unsafe -location;
        unsafe -locationNumber;
        unsafe -maxTransferRate;
        unsafe -isDepth24Stencil8PixelFormatSupported;
        unsafe -readWriteTextureSupport;
        unsafe -argumentBuffersSupport;
        unsafe -supports32BitFloatFiltering;
        unsafe -supports32BitMSAA;
        unsafe -supportsQueryTextureLOD;
        unsafe -supportsBCTextureCompression;
        unsafe -supportsPullModelInterpolation;
        unsafe -supportsShaderBarycentricCoordinates;
        unsafe -currentAllocatedSize;
        unsafe -newCommandQueue;
        unsafe -newCommandQueueWithMaxCommandBufferCount;
        unsafe -heapTextureSizeAndAlignWithDescriptor;
        unsafe -heapBufferSizeAndAlignWithLength_options;
        unsafe -newHeapWithDescriptor;
        unsafe -newBufferWithLength_options;
        unsafe -newDepthStencilStateWithDescriptor;
        unsafe -newTextureWithDescriptor;
        unsafe -newSamplerStateWithDescriptor;
        unsafe -newDefaultLibrary;
        unsafe -newLibraryWithFile_error;
        unsafe -newLibraryWithSource_options_error;
        unsafe -newRenderPipelineStateWithDescriptor_error;
        unsafe -newComputePipelineStateWithFunction_error;
        unsafe -newFence;
        unsafe -supportsFeatureSet;
        unsafe -supportsFamily;
        unsafe -supportsTextureSampleCount;
        unsafe -minimumLinearTextureAlignmentForPixelFormat;
        unsafe -minimumTextureBufferAlignmentForPixelFormat;
        unsafe -maxThreadgroupMemoryLength;
        unsafe -maxArgumentBufferSamplerCount;
        unsafe -newArgumentEncoderWithArguments;
        unsafe -newEvent;
        unsafe -newSharedEvent;
        unsafe -maxBufferLength;
        unsafe -supportsCounterSampling;
        unsafe -supportsVertexAmplificationCount;
        unsafe -supportsDynamicLibraries;
        unsafe -newDynamicLibrary_error;
        unsafe -newDynamicLibraryWithURL_error;
        unsafe -newBinaryArchiveWithDescriptor_error;
        unsafe -supportsRaytracing;
        unsafe -accelerationStructureSizesWithDescriptor;
        unsafe -newAccelerationStructureWithSize;
        unsafe -supportsFunctionPointers;
    }

    protocol MTLDrawable {
        unsafe -present;
        unsafe -drawableID;
    }

    protocol MTLCommandEncoder {
        unsafe -label;
        unsafe -setLabel;
        unsafe -endEncoding;
        unsafe -insertDebugSignpost;
        unsafe -pushDebugGroup;
        unsafe -popDebugGroup;
    }

    protocol MTLParallelRenderCommandEncoder {
        unsafe -renderCommandEncoder;
    }

    // TODO: Verify that offset out-of-bounds is sound.
    // TODO: Verify that index out-of-bounds is sound.
    protocol MTLRenderCommandEncoder {
        unsafe -setRenderPipelineState;
        // unsafe -setVertexBuffer_offset_atIndex;
        // unsafe -setVertexTexture_atIndex;
        // ...
        unsafe -setViewport;
        unsafe -setFrontFacingWinding;
        unsafe -setCullMode;
        unsafe -setDepthClipMode;
        unsafe -setDepthBias_slopeScale_clamp;
        unsafe -setScissorRect;
        unsafe -setTriangleFillMode;
        // unsafe -setFragmentBuffer_...;
        // ...
        unsafe -setBlendColorRed_green_blue_alpha;
        unsafe -setDepthStencilState;
        unsafe -setStencilReferenceValue;
        unsafe -setStencilFrontReferenceValue_backReferenceValue;
        unsafe -setVisibilityResultMode_offset;
        // unsafe -drawPrimitives_...;
        // ...
        unsafe -updateFence_afterStages;
        unsafe -waitForFence_beforeStages;
        // unsafe -setThreadgroupMemoryLength_offset_atIndex;
        unsafe -useResource_usage;
        unsafe -useResource_usage_stages;
        unsafe -useHeap;
        unsafe -useHeap_stages;
    }

    // TODO: Verify out-of-bounds access is sound.
    protocol MTLBlitCommandEncoder {
        unsafe -synchronizeResource;
        // unsafe -copyFrom...;
        unsafe -generateMipmapsForTexture;
        unsafe -fillBuffer_range_value;
        unsafe -updateFence;
        unsafe -waitForFence;
        unsafe -optimizeContentsForGPUAccess;
        // unsafe -optimizeContentsForGPUAccess_slice_level;
    }

    // TODO: Verify out-of-bounds access is sound.
    protocol MTLComputeCommandEncoder {
        unsafe -setComputePipelineState;
        // unsafe -setBuffer_...;
        // unsafe -setIntersectionFunctionTable_atBufferIndex;
        unsafe -dispatchThreadgroups_threadsPerThreadgroup;
        // unsafe -dispatchThreadgroupsWithIndirectBuffer_indirectBufferOffset_threadsPerThreadgroup;
        unsafe -dispatchThreads_threadsPerThreadgroup;
        unsafe -updateFence;
        unsafe -waitForFence;
        unsafe -useResource_usage;
        unsafe -useHeap;
    }

    // TODO: Verify out-of-bounds access is sound.
    protocol MTLArgumentEncoder {
        unsafe -encodedLength;
        unsafe -alignment;
        // unsafe -setArgumentBuffer_offset;
        // ...
    }

    class MTLHeapDescriptor {
        unsafe -size;
        unsafe -setSize;
        unsafe -storageMode;
        unsafe -setStorageMode;
        unsafe -cpuCacheMode;
        unsafe -setCpuCacheMode;
        unsafe -hazardTrackingMode;
        unsafe -setHazardTrackingMode;
        unsafe -resourceOptions;
        unsafe -setResourceOptions;
        unsafe -type_;
        unsafe -setType;
    }

    protocol MTLHeap {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -storageMode;
        unsafe -cpuCacheMode;
        unsafe -hazardTrackingMode;
        unsafe -resourceOptions;
        unsafe -size;
        unsafe -usedSize;
        unsafe -currentAllocatedSize;
        unsafe -maxAvailableSizeWithAlignment;
        unsafe -newBufferWithLength_options;
        unsafe -newTextureWithDescriptor;
        unsafe -setPurgeableState;
        unsafe -type_;
        // TODO: Verify that offset out-of-bounds is sound.
        // unsafe -newBufferWithLength_options_offset;
        // unsafe -newTextureWithDescriptor_offset;
    }

    class MTLIndirectCommandBufferDescriptor {
        unsafe -commandTypes;
        unsafe -setCommandTypes;
        unsafe -inheritPipelineState;
        unsafe -setInheritPipelineState;
        unsafe -inheritBuffers;
        unsafe -setInheritBuffers;
        unsafe -maxVertexBufferBindCount;
        unsafe -setMaxVertexBufferBindCount;
        unsafe -maxFragmentBufferBindCount;
        unsafe -setMaxFragmentBufferBindCount;
        unsafe -maxKernelBufferBindCount;
        unsafe -setMaxKernelBufferBindCount;
    }

    protocol MTLIndirectCommandBuffer {
        unsafe -size;
        // TODO: Verify that index out-of-bounds is sound.
        // unsafe -resetWithRange;
        // unsafe -indirectRenderCommandAtIndex;
        // unsafe -indirectComputeCommandAtIndex;
    }

    class MTLVertexAttribute {
        unsafe -name;
        unsafe -attributeIndex;
        unsafe -attributeType;
        unsafe -isActive;
        unsafe -isPatchData;
        unsafe -isPatchControlPointData;
    }

    class MTLAttribute {
        unsafe -name;
        unsafe -attributeIndex;
        unsafe -attributeType;
        unsafe -isActive;
        unsafe -isPatchData;
        unsafe -isPatchControlPointData;
    }

    class MTLFunctionConstant {
        unsafe -name;
        unsafe -type_;
        unsafe -index;
        unsafe -required;
    }

    class MTLFunctionDescriptor {
        unsafe +new;
        unsafe +functionDescriptor;
        unsafe -name;
        unsafe -setName;
        unsafe -specializedName;
        unsafe -setSpecializedName;
        unsafe -constantValues;
        unsafe -setConstantValues;
        unsafe -options;
        unsafe -setOptions;
    }

    protocol MTLFunction {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -functionType;
        unsafe -patchType;
        unsafe -patchControlPointCount;
        unsafe -vertexAttributes;
        unsafe -stageInputAttributes;
        unsafe -name;
        unsafe -functionConstantsDictionary;
        // TODO: Verify that index out-of-bounds is sound.
        // unsafe -newArgumentEncoderWithBufferIndex;
        unsafe -options;
        unsafe -label;
    }

    protocol MTLFunctionHandle {
        unsafe -functionType;
        unsafe -name;
        unsafe -device;
    }

    class MTLFunctionConstantValues {
        unsafe +new;
        unsafe -reset;
    }

    class MTLCompileOptions {
        unsafe +new;
        unsafe -preprocessorMacros;
        unsafe -fastMathEnabled;
        unsafe -setFastMathEnabled;
        unsafe -languageVersion;
        unsafe -setLanguageVersion;
        unsafe -libraryType;
        unsafe -setLibraryType;
        unsafe -installName;
        unsafe -libraries;
        unsafe -setLibraries;
        unsafe -preserveInvariance;
        unsafe -setPreserveInvariance;
    }

    protocol MTLLibrary {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -newFunctionWithName;
        unsafe -newFunctionWithName_constantValues_error;
        unsafe -newFunctionWithDescriptor_error;
        unsafe -newIntersectionFunctionWithDescriptor_error;
        unsafe -functionNames;
        unsafe -type_;
        unsafe -installName;
    }

    protocol MTLDynamicLibrary {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -installName;
        unsafe -serializeToURL_error;
    }

    class MTLBinaryArchiveDescriptor {
        unsafe +new;
        unsafe -url;
        unsafe -setUrl;
    }

    protocol MTLBinaryArchive {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -addComputePipelineFunctionsWithDescriptor_error;
        unsafe -addRenderPipelineFunctionsWithDescriptor_error;
        unsafe -serializeToURL_error;
    }

    class MTLLinkedFunctions {
        unsafe +new;
        unsafe -linkedFunctions;
        unsafe -functions;
        unsafe -setFunctions;
        unsafe -binaryFunctions;
        unsafe -setBinaryFunctions;
        unsafe -groups;
        unsafe -setGroups;
        unsafe -privateFunctions;
        unsafe -setPrivateFunctions;
    }

    class MTLRenderPassAttachmentDescriptor {
        unsafe -texture;
        unsafe -setTexture;
        unsafe -level;
        unsafe -setLevel;
        unsafe -slice;
        unsafe -setSlice;
        unsafe -depthPlane;
        unsafe -setDepthPlane;
        unsafe -resolveTexture;
        unsafe -setResolveTexture;
        unsafe -resolveLevel;
        unsafe -setResolveLevel;
        unsafe -resolveSlice;
        unsafe -setResolveSlice;
        unsafe -resolveDepthPlane;
        unsafe -setResolveDepthPlane;
        unsafe -loadAction;
        unsafe -setLoadAction;
        unsafe -storeAction;
        unsafe -setStoreAction;
        unsafe -storeActionOptions;
        unsafe -setStoreActionOptions;
    }

    class MTLRenderPassColorAttachmentDescriptor {
        unsafe +new;
        unsafe -clearColor;
        unsafe -setClearColor;
    }

    class MTLRenderPassDepthAttachmentDescriptor {
        unsafe -clearDepth;
        unsafe -setClearDepth;
        unsafe -depthResolveFilter;
        unsafe -setDepthResolveFilter;
    }

    class MTLRenderPassStencilAttachmentDescriptor {
        unsafe -clearStencil;
        unsafe -setClearStencil;
        unsafe -stencilResolveFilter;
        unsafe -setStencilResolveFilter;
    }

    // TODO: Verify that index out-of-bounds is sound.
    class MTLRenderPassSampleBufferAttachmentDescriptor {
        unsafe -sampleBuffer;
        unsafe -setSampleBuffer;
        unsafe -startOfVertexSampleIndex;
        // unsafe -setStartOfVertexSampleIndex;
        unsafe -endOfVertexSampleIndex;
        // unsafe -setEndOfVertexSampleIndex;
        unsafe -startOfFragmentSampleIndex;
        // unsafe -setStartOfFragmentSampleIndex;
        unsafe -endOfFragmentSampleIndex;
        // unsafe -setEndOfFragmentSampleIndex;
    }

    // TODO: Verify that index out-of-bounds is sound.
    class MTLRenderPassDescriptor {
        unsafe +renderPassDescriptor;
        unsafe -colorAttachments;
        unsafe -depthAttachment;
        unsafe -setDepthAttachment;
        unsafe -stencilAttachment;
        unsafe -setStencilAttachment;
        unsafe -visibilityResultBuffer;
        unsafe -setVisibilityResultBuffer;
        unsafe -renderTargetArrayLength;
        // unsafe -setRenderTargetArrayLength;
        unsafe -imageblockSampleLength;
        // unsafe -setImageblockSampleLength;
        unsafe -threadgroupMemoryLength;
        // unsafe -setThreadgroupMemoryLength;
        unsafe -tileWidth;
        unsafe -setTileWidth;
        unsafe -tileHeight;
        unsafe -setTileHeight;
        unsafe -defaultRasterSampleCount;
        unsafe -setDefaultRasterSampleCount;
        unsafe -renderTargetWidth;
        unsafe -setRenderTargetWidth;
        unsafe -renderTargetHeight;
        unsafe -setRenderTargetHeight;
        unsafe -rasterizationRateMap;
        unsafe -setRasterizationRateMap;
        unsafe -sampleBufferAttachments;
    }

    protocol MTLResource {
        unsafe -label;
        unsafe -setLabel;
        unsafe -device;
        unsafe -cpuCacheMode;
        unsafe -storageMode;
        unsafe -hazardTrackingMode;
        unsafe -resourceOptions;
        unsafe -setPurgeableState;
        unsafe -heap;
        unsafe -heapOffset;
        unsafe -allocatedSize;
        unsafe -isAliasable;
    }

    class MTLSamplerDescriptor {
        unsafe +new;
        unsafe -minFilter;
        unsafe -setMinFilter;
        unsafe -magFilter;
        unsafe -setMagFilter;
        unsafe -mipFilter;
        unsafe -setMipFilter;
        unsafe -maxAnisotropy;
        unsafe -setMaxAnisotropy;
        unsafe -sAddressMode;
        unsafe -setSAddressMode;
        unsafe -tAddressMode;
        unsafe -setTAddressMode;
        unsafe -rAddressMode;
        unsafe -setRAddressMode;
        unsafe -borderColor;
        unsafe -setBorderColor;
        unsafe -normalizedCoordinates;
        unsafe -setNormalizedCoordinates;
        unsafe -lodMinClamp;
        unsafe -setLodMinClamp;
        unsafe -lodMaxClamp;
        unsafe -setLodMaxClamp;
        unsafe -lodAverage;
        unsafe -setLodAverage;
        unsafe -compareFunction;
        unsafe -setCompareFunction;
        unsafe -supportArgumentBuffers;
        unsafe -setSupportArgumentBuffers;
        unsafe -label;
        unsafe -setLabel;
    }

    protocol MTLSamplerState {
        unsafe -label;
        unsafe -device;
    }

    protocol MTLEvent {
        unsafe -device;
        unsafe -label;
        unsafe -setLabel;
    }

    class MTLSharedEventListener {
        unsafe +new;
    }

    class MTLSharedEvent {
        unsafe -newSharedEventHandle;
        unsafe -signaledValue;
        unsafe -setSignaledValue;
    }

    class MTLSharedEventHandle {
        unsafe -label;
    }

    protocol MTLFence {
        unsafe -device;
        unsafe -label;
        unsafe -setLabel;
    }

    class MTLSharedTextureHandle {
        unsafe -device;
        unsafe -label;
    }

    // Sizes must be >= 1
    class MTLTextureDescriptor {
        // unsafe -texture2DDescriptorWithPixelFormat_width_height_mipmapped;
        // unsafe -textureCubeDescriptorWithPixelFormat_size_mipmapped;
        // unsafe -textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage;
        unsafe -textureType;
        unsafe -setTextureType;
        unsafe -pixelFormat;
        unsafe -setPixelFormat;
        unsafe -width;
        // unsafe -setWidth;
        unsafe -height;
        // unsafe -setHeight;
        unsafe -depth;
        // unsafe -setDepth;
        unsafe -mipmapLevelCount;
        // unsafe -setMipmapLevelCount;
        unsafe -sampleCount;
        // unsafe -setSampleCount;
        unsafe -arrayLength;
        // unsafe -setArrayLength;
        unsafe -resourceOptions;
        unsafe -setResourceOptions;
        unsafe -cpuCacheMode;
        unsafe -setCpuCacheMode;
        unsafe -storageMode;
        unsafe -setStorageMode;
        unsafe -hazardTrackingMode;
        unsafe -setHazardTrackingMode;
        unsafe -usage;
        unsafe -setUsage;
        unsafe -allowGPUOptimizedContents;
        unsafe -setAllowGPUOptimizedContents;
        unsafe -swizzle;
        unsafe -setSwizzle;
    }

    protocol MTLTexture {
        unsafe -rootResource;
        unsafe -parentTexture;
        unsafe -parentRelativeLevel;
        unsafe -parentRelativeSlice;
        unsafe -buffer;
        unsafe -bufferOffset;
        unsafe -bufferBytesPerRow;
        unsafe -iosurfacePlane;
        unsafe -textureType;
        unsafe -pixelFormat;
        unsafe -width;
        unsafe -height;
        unsafe -depth;
        unsafe -mipmapLevelCount;
        unsafe -sampleCount;
        unsafe -arrayLength;
        unsafe -usage;
        unsafe -isShareable;
        unsafe -isFramebufferOnly;
        unsafe -firstMipmapInTail;
        unsafe -tailSizeInBytes;
        unsafe -isSparse;
        unsafe -allowGPUOptimizedContents;
        unsafe -newTextureViewWithPixelFormat;
        unsafe -newSharedTextureHandle;
        unsafe -remoteStorageTexture;
        unsafe -swizzle;
        // TODO: Verify that index out-of-bounds is sound.
        // unsafe -newTextureViewWithPixelFormat_textureType_levels_slices;
        // unsafe -newTextureViewWithPixelFormat_textureType_levels_slices_swizzle;
    }

    class MTLBufferLayoutDescriptor {
        unsafe -stride;
        unsafe -setStride;
        unsafe -stepFunction;
        unsafe -setStepFunction;
        unsafe -stepRate;
        unsafe -setStepRate;
    }

    class MTLAttributeDescriptor {
        unsafe -format;
        unsafe -setFormat;
        unsafe -offset;
        unsafe -setOffset;
        unsafe -bufferIndex;
        // TODO: Verify that index out-of-bounds is sound.
        // unsafe -setBufferIndex;
    }

    class MTLVertexBufferLayoutDescriptor {
        unsafe +new;
        unsafe -stride;
        // Must be a multiple of 4
        // unsafe -setStride;
        unsafe -stepFunction;
        // setStepFunction and setStepRate must be done in lockstep
        // unsafe -setStepFunction;
        unsafe -stepRate;
        // unsafe -setStepRate;
    }

    class MTLVertexAttributeDescriptor {
        unsafe +new;
        unsafe -format;
        unsafe -setFormat;
        unsafe -offset;
        // Must be a multiple of 4
        // unsafe -setOffset;
        unsafe -bufferIndex;
        // TODO: Verify that index out-of-bounds is sound.
        // unsafe -setBufferIndex;
    }

    class MTLVertexDescriptor {
        unsafe +vertexDescriptor;
        unsafe -layouts;
        unsafe -attributes;
        unsafe -reset;
    }

    // TODO:
    class MTLStageInputOutputDescriptor {
        unsafe +stageInputOutputDescriptor;
        unsafe -layouts;
        unsafe -attributes;
        unsafe -indexType;
        // unsafe -setIndexType;
        unsafe -indexBufferIndex;
        // unsafe -setIndexBufferIndex;
        unsafe -reset;
    }

    class MTLPipelineBufferDescriptor {
        unsafe -mutability;
        unsafe -setMutability;
    }

    class MTLComputePipelineReflection {
        unsafe -arguments;
    }

    class MTLComputePipelineDescriptor {
        unsafe +new;
        unsafe -label;
        unsafe -setLabel;
        unsafe -computeFunction;
        unsafe -setComputeFunction;
        unsafe -threadGroupSizeIsMultipleOfThreadExecutionWidth;
        // unsafe -setThreadGroupSizeIsMultipleOfThreadExecutionWidth;
        unsafe -maxTotalThreadsPerThreadgroup;
        unsafe -setMaxTotalThreadsPerThreadgroup;
        unsafe -stageInputDescriptor;
        unsafe -setStageInputDescriptor;
        unsafe -buffers;
        unsafe -supportIndirectCommandBuffers;
        unsafe -setSupportIndirectCommandBuffers;
        unsafe -insertLibraries;
        unsafe -setInsertLibraries;
        unsafe -preloadedLibraries;
        unsafe -setPreloadedLibraries;
        unsafe -binaryArchives;
        unsafe -setBinaryArchives;
        unsafe -reset;
        unsafe -linkedFunctions;
        unsafe -setLinkedFunctions;
        unsafe -supportAddingBinaryFunctions;
        unsafe -setSupportAddingBinaryFunctions;
        unsafe -maxCallStackDepth;
        unsafe -setMaxCallStackDepth;
    }

    protocol MTLComputePipelineState {
        unsafe -label;
        unsafe -device;
        unsafe -maxTotalThreadsPerThreadgroup;
        unsafe -threadExecutionWidth;
        unsafe -staticThreadgroupMemoryLength;
        // unsafe -imageblockMemoryLengthForDimensions;
        unsafe -supportIndirectCommandBuffers;
        unsafe -functionHandleWithFunction;
        unsafe -newComputePipelineStateWithAdditionalBinaryFunctions_error;
        unsafe -newVisibleFunctionTableWithDescriptor;
        unsafe -newIntersectionFunctionTableWithDescriptor;
    }

    class MTLRenderPipelineReflection {
        unsafe -vertexArguments;
        unsafe -fragmentArguments;
        unsafe -tileArguments;
    }

    class MTLRenderPipelineDescriptor {
        unsafe +new;
        unsafe -label;
        unsafe -setLabel;
        unsafe -vertexFunction;
        unsafe -setVertexFunction;
        unsafe -fragmentFunction;
        unsafe -setFragmentFunction;
        unsafe -vertexDescriptor;
        unsafe -setVertexDescriptor;
        unsafe -sampleCount;
        unsafe -setSampleCount;
        unsafe -rasterSampleCount;
        unsafe -setRasterSampleCount;
        unsafe -isAlphaToCoverageEnabled;
        unsafe -setAlphaToCoverageEnabled;
        unsafe -isAlphaToOneEnabled;
        unsafe -setAlphaToOneEnabled;
        unsafe -isRasterizationEnabled;
        unsafe -setRasterizationEnabled;
        unsafe -maxVertexAmplificationCount;
        // Call supportsVertexAmplificationCount: on device first
        // unsafe -setMaxVertexAmplificationCount;
        unsafe -colorAttachments;
        unsafe -depthAttachmentPixelFormat;
        unsafe -setDepthAttachmentPixelFormat;
        unsafe -stencilAttachmentPixelFormat;
        unsafe -setStencilAttachmentPixelFormat;
        unsafe -inputPrimitiveTopology;
        // Must be specified when layered rendering is enabled
        // unsafe -setInputPrimitiveTopology;
        unsafe -tessellationPartitionMode;
        // Affects maxTessellationFactor
        // unsafe -setTessellationPartitionMode;
        unsafe -maxTessellationFactor;
        // Must be between 16 and 64 and depends on tessellationPartitionMode
        // unsafe -setMaxTessellationFactor;
        unsafe -isTessellationFactorScaleEnabled;
        unsafe -setTessellationFactorScaleEnabled;
        unsafe -tessellationFactorFormat;
        unsafe -setTessellationFactorFormat;
        unsafe -tessellationControlPointIndexType;
        // Requires specific values when using indexed control points
        // unsafe -setTessellationControlPointIndexType;
        unsafe -tessellationFactorStepFunction;
        unsafe -setTessellationFactorStepFunction;
        unsafe -tessellationOutputWindingOrder;
        unsafe -setTessellationOutputWindingOrder;
        unsafe -vertexBuffers;
        unsafe -fragmentBuffers;
        unsafe -supportIndirectCommandBuffers;
        unsafe -setSupportIndirectCommandBuffers;
        unsafe -binaryArchives;
        unsafe -setBinaryArchives;
        unsafe -vertexPreloadedLibraries;
        unsafe -setVertexPreloadedLibraries;
        unsafe -fragmentPreloadedLibraries;
        unsafe -setFragmentPreloadedLibraries;
        unsafe -vertexLinkedFunctions;
        unsafe -setVertexLinkedFunctions;
        unsafe -fragmentLinkedFunctions;
        unsafe -setFragmentLinkedFunctions;
        unsafe -supportAddingVertexBinaryFunctions;
        unsafe -setSupportAddingVertexBinaryFunctions;
        unsafe -supportAddingFragmentBinaryFunctions;
        unsafe -setSupportAddingFragmentBinaryFunctions;
        unsafe -maxVertexCallStackDepth;
        unsafe -setMaxVertexCallStackDepth;
        unsafe -maxFragmentCallStackDepth;
        unsafe -setMaxFragmentCallStackDepth;
        unsafe -reset;
    }

    class MTLRenderPipelineState {
        unsafe -label;
        unsafe -device;
        unsafe -maxTotalThreadsPerThreadgroup;
        unsafe -threadgroupSizeMatchesTileSize;
        unsafe -imageblockSampleLength;
        unsafe -imageblockMemoryLengthForDimensions;
        unsafe -supportIndirectCommandBuffers;
        unsafe -functionHandleWithFunction_stage;
        unsafe -newVisibleFunctionTableWithDescriptor_stage;
        unsafe -newIntersectionFunctionTableWithDescriptor_stage;
        unsafe -newRenderPipelineStateWithAdditionalBinaryFunctions_error;
    }

    class MTLRenderPipelineColorAttachmentDescriptor {
        unsafe -pixelFormat;
        unsafe -setPixelFormat;
        unsafe -isBlendingEnabled;
        unsafe -setBlendingEnabled;
        unsafe -sourceRGBBlendFactor;
        unsafe -setSourceRGBBlendFactor;
        unsafe -destinationRGBBlendFactor;
        unsafe -setDestinationRGBBlendFactor;
        unsafe -rgbBlendOperation;
        unsafe -setRgbBlendOperation;
        unsafe -sourceAlphaBlendFactor;
        unsafe -setSourceAlphaBlendFactor;
        unsafe -destinationAlphaBlendFactor;
        unsafe -setDestinationAlphaBlendFactor;
        unsafe -alphaBlendOperation;
        unsafe -setAlphaBlendOperation;
        unsafe -writeMask;
        unsafe -setWriteMask;
    }
}
