// Header containing types and enum constants shared between Metal shaders and Rust source code
//
// These are used to generate Rust types in the `build.rs` build script.

#ifndef shader_types_h
#define shader_types_h

// Used to get the definition of vector_float2
// If you need to import more types, simd is typically located at:
// /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/simd
#include <simd/vector_types.h>

typedef enum VertexInputIndex {
    VertexInputIndexVertices = 0,
    VertexInputIndexViewportSize = 1,
} VertexInputIndex;

typedef enum TextureIndex {
    TextureIndexBaseColor = 0
} TextureIndex;

typedef struct {
    // (0, 0) is the center of the screen
    // (-viewport_size / 2, 0) is the middle of the furthest left column of screen pixels.
    // (0, viewport_size / 2) is the middle of the furthest top row of screen pixels.
    vector_float2 position;
    // (0,0) is the top left corner of the image
    // (1,1) is the bottom right corner of the image
    vector_float2 texture_coord;
} TexturedVertex;

#endif
