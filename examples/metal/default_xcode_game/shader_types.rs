//! Type aliases shared with Metal.

#![allow(non_upper_case_globals)]
use super::matrix::Float4x4;

pub type EnumBackingType = usize;

pub type BufferIndex = EnumBackingType;
pub const BufferIndexMeshPositions: BufferIndex = 0;
pub const BufferIndexMeshGenerics: BufferIndex = 1;
pub const BufferIndexUniforms: BufferIndex = 2;

pub type VertexAttribute = EnumBackingType;
pub const VertexAttributePosition: VertexAttribute = 0;
pub const VertexAttributeTexcoord: VertexAttribute = 1;

pub type TextureIndex = EnumBackingType;
pub const TextureIndexColor: TextureIndex = 0;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Uniforms {
    pub projection_matrix: Float4x4,
    pub model_view_matrix: Float4x4,
}
