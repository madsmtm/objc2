//
//  Created by Sergey Reznik on 9/15/18.
//  Copyright © 2018 Serhii Rieznik. All rights reserved.
//

// Taken from https://github.com/sergeyreznik/metal-ray-tracer/tree/part-1/source/Shaders
// MIT License https://github.com/sergeyreznik/metal-ray-tracer/blob/part-1/LICENSE

// HACK: Ideally, we'd do an include like:
// #include <MetalPerformanceShaders/MetalPerformanceShaders.h>
//
// But since we're compiling this at runtime for the convenience of the
// example, we need to define these types manually.

typedef struct {
    packed_float3 origin;
    float minDistance;
    packed_float3 direction;
    float maxDistance;
} MPSRayOriginMinDistanceDirectionMaxDistance;

typedef struct {
    float distance;
    unsigned int primitiveIndex;
    vector_float2 coordinates;
} MPSIntersectionDistancePrimitiveIndexCoordinates;

// END HACK

using Ray = MPSRayOriginMinDistanceDirectionMaxDistance;
using Intersection = MPSIntersectionDistancePrimitiveIndexCoordinates;

kernel void generateRays(
    device Ray* rays [[buffer(0)]],
    uint2 coordinates [[thread_position_in_grid]],
    uint2 size [[threads_per_grid]])
{
    float2 uv = float2(coordinates) / float2(size - 1);

    uint rayIndex = coordinates.x + coordinates.y * size.x;
    rays[rayIndex].origin = packed_float3(uv.x, uv.y, -1.0);
    rays[rayIndex].direction = packed_float3(0.0, 0.0, 1.0);
    rays[rayIndex].minDistance = 0.0f;
    rays[rayIndex].maxDistance = 2.0f;
}
