#include <metal_stdlib>
#include <simd/simd.h>

using namespace metal;

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

typedef struct {
    // The [[position]] attribute qualifier of this member indicates this
    // value is the clip space position of the vertex when this structure
    // is returned from the vertex shader.
    float4 position [[position]];

    // Since this member does not have a special attribute qualifier, the
    // rasterizer will interpolate its value with values of other verticse
    // making up the triangle and pass that interpolated value to the
    // fragment shader for each fragment of that triangle.
    float2 texture_coord;
} RasterizerData;

vertex RasterizerData quad_vertex(
	uint vertex_id [[ vertex_id ]],
	constant TexturedVertex *vert_array [[ buffer(VertexInputIndexVertices) ]],
	constant uint2 *viewport_size_ptr [[ buffer(VertexInputIndexViewportSize) ]]
) {
  RasterizerData out;

  float2 pixel_space_pos = vert_array[vertex_id].position.xy;
  float2 viewport_size = float2(*viewport_size_ptr);

  float2 clip_space_pos = (pixel_space_pos / viewport_size) * 2.0;

  out.position = float4(clip_space_pos, 0.0, 1.0);
  out.texture_coord = vert_array[vertex_id].texture_coord;

  return out;
}

fragment float4 sampling_shader(
  RasterizerData in [[stage_in]],
  texture2d<half> color_texture [[ texture(TextureIndexBaseColor) ]]
 ) {
  constexpr sampler texture_sampler (mag_filter::linear, min_filter::linear);

  const half4 color_sample = color_texture.sample(texture_sampler, in.texture_coord);

  return float4(color_sample);
}
