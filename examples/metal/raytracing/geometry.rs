use std::{ptr::NonNull, rc::Rc};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use objc2::{rc::Retained, runtime::ProtocolObject, Message};
use objc2_foundation::{ns_string, NSRange, NSString, NSUInteger};
use objc2_metal::{
    MTLAccelerationStructureBoundingBoxGeometryDescriptor,
    MTLAccelerationStructureGeometryDescriptor, MTLAccelerationStructureTriangleGeometryDescriptor,
    MTLBuffer, MTLDevice, MTLIndexType, MTLResource, MTLResourceOptions,
};

pub const GEOMETRY_MASK_TRIANGLE: u32 = 1;
pub const GEOMETRY_MASK_SPHERE: u32 = 2;
pub const GEOMETRY_MASK_LIGHT: u32 = 4;

pub const FACE_MASK_NONE: u16 = 0;
pub const FACE_MASK_NEGATIVE_X: u16 = 1 << 0;
pub const FACE_MASK_POSITIVE_X: u16 = 1 << 1;
pub const FACE_MASK_NEGATIVE_Y: u16 = 1 << 2;
pub const FACE_MASK_POSITIVE_Y: u16 = 1 << 3;
pub const FACE_MASK_NEGATIVE_Z: u16 = 1 << 4;
pub const FACE_MASK_POSITIVE_Z: u16 = 1 << 5;
pub const FACE_MASK_ALL: u16 = (1 << 6) - 1;

pub trait Geometry {
    fn upload_to_buffers(&mut self);
    fn clear(&mut self);
    fn get_geometry_descriptor(&self) -> Retained<MTLAccelerationStructureGeometryDescriptor>;
    fn get_resources(&self) -> Vec<&ProtocolObject<dyn MTLResource>>;
    fn get_intersection_function_name(&self) -> Option<&NSString> {
        None
    }
}

pub fn compute_triangle_normal(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> Vec3 {
    let e1 = Vec3::normalize(*v1 - *v0);
    let e2 = Vec3::normalize(*v2 - *v0);
    Vec3::cross(e1, e2)
}

#[derive(Default)]
#[repr(C)]
pub struct Triangle {
    pub normals: [Vec4; 3],
    pub colours: [Vec4; 3],
}

pub fn get_managed_buffer_storage_mode() -> MTLResourceOptions {
    MTLResourceOptions::StorageModeManaged
}

pub struct TriangleGeometry {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub name: String,
    pub index_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub vertex_position_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub vertex_normal_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub vertex_colour_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub per_primitive_data_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub indices: Vec<u16>,
    pub vertices: Vec<Vec4>,
    pub normals: Vec<Vec4>,
    pub colours: Vec<Vec4>,
    pub triangles: Vec<Triangle>,
}

impl TriangleGeometry {
    pub fn new(device: &ProtocolObject<dyn MTLDevice>, name: String) -> Self {
        Self {
            device: device.retain(),
            name,
            index_buffer: None,
            vertex_position_buffer: None,
            vertex_normal_buffer: None,
            vertex_colour_buffer: None,
            per_primitive_data_buffer: None,
            indices: Vec::new(),
            vertices: Vec::new(),
            normals: Vec::new(),
            colours: Vec::new(),
            triangles: Vec::new(),
        }
    }

    pub fn add_cube_face_with_cube_vertices(
        &mut self,
        cube_vertices: &[Vec3],
        colour: Vec3,
        [i0, i1, i2, i3]: [u16; 4],
        inward_normals: bool,
    ) {
        let v0 = cube_vertices[i0 as usize];
        let v1 = cube_vertices[i1 as usize];
        let v2 = cube_vertices[i2 as usize];
        let v3 = cube_vertices[i3 as usize];

        let n0 = compute_triangle_normal(&v0, &v1, &v2) * if inward_normals { -1f32 } else { 1f32 };
        let n1 = compute_triangle_normal(&v0, &v2, &v3) * if inward_normals { -1f32 } else { 1f32 };

        let first_index = self.indices.len();
        let base_index = self.vertices.len() as u16;

        self.indices.push(base_index);
        self.indices.push(base_index + 1);
        self.indices.push(base_index + 2);
        self.indices.push(base_index);
        self.indices.push(base_index + 2);
        self.indices.push(base_index + 3);

        self.vertices.push(From::from((v0, 0.0)));
        self.vertices.push(From::from((v1, 0.0)));
        self.vertices.push(From::from((v2, 0.0)));
        self.vertices.push(From::from((v3, 0.0)));

        self.normals
            .push(From::from((Vec3::normalize(n0 + n1), 0.0)));
        self.normals.push(From::from((n0, 0.0)));
        self.normals
            .push(From::from((Vec3::normalize(n0 + n1), 0.0)));
        self.normals.push(From::from((n1, 0.0)));

        for _ in 0..4 {
            self.colours.push(From::from((colour, 0.0)));
        }

        for triangle_index in 0..2 {
            let mut triangle = Triangle::default();
            for i in 0..3 {
                let index = self.indices[first_index + triangle_index * 3 + i];
                triangle.normals[i] = self.normals[index as usize];
                triangle.colours[i] = self.colours[index as usize];
            }
            self.triangles.push(triangle);
        }
    }

    pub fn add_cube_with_faces(
        &mut self,
        face_mask: u16,
        colour: Vec3,
        transform: Mat4,
        inward_normals: bool,
    ) {
        let mut cube_vertices = [
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
        ];

        for v in &mut cube_vertices {
            let transformed_vertex = v.extend(1.0);
            let transformed_vertex = transform * transformed_vertex;
            *v = transformed_vertex.xyz();
        }

        const CUBE_INDICES: [[u16; 4]; 6] = [
            [0, 4, 6, 2],
            [1, 3, 7, 5],
            [0, 1, 5, 4],
            [2, 6, 7, 3],
            [0, 2, 3, 1],
            [4, 5, 7, 6],
        ];

        for (face, indices) in CUBE_INDICES.into_iter().enumerate() {
            if face_mask & (1 << face) != 0 {
                self.add_cube_face_with_cube_vertices(
                    &cube_vertices,
                    colour,
                    indices,
                    inward_normals,
                );
            }
        }
    }
}

impl Geometry for TriangleGeometry {
    fn upload_to_buffers(&mut self) {
        unsafe {
            self.index_buffer = Some(
                self.device
                    .newBufferWithBytes_length_options(
                        NonNull::new(self.indices.as_ptr().cast_mut().cast()).unwrap(),
                        size_of_val(self.indices.as_slice()),
                        get_managed_buffer_storage_mode(),
                    )
                    .unwrap(),
            );
            self.vertex_position_buffer = Some(
                self.device
                    .newBufferWithBytes_length_options(
                        NonNull::new(self.vertices.as_ptr().cast_mut().cast()).unwrap(),
                        size_of_val(self.vertices.as_slice()),
                        get_managed_buffer_storage_mode(),
                    )
                    .unwrap(),
            );
            self.vertex_normal_buffer = Some(
                self.device
                    .newBufferWithBytes_length_options(
                        NonNull::new(self.normals.as_ptr().cast_mut().cast()).unwrap(),
                        size_of_val(self.normals.as_slice()),
                        get_managed_buffer_storage_mode(),
                    )
                    .unwrap(),
            );
            self.vertex_colour_buffer = Some(
                self.device
                    .newBufferWithBytes_length_options(
                        NonNull::new(self.colours.as_ptr().cast_mut().cast()).unwrap(),
                        size_of_val(self.colours.as_slice()),
                        get_managed_buffer_storage_mode(),
                    )
                    .unwrap(),
            );
            self.per_primitive_data_buffer = Some(
                self.device
                    .newBufferWithBytes_length_options(
                        NonNull::new(self.triangles.as_ptr().cast_mut().cast()).unwrap(),
                        size_of_val(self.triangles.as_slice()),
                        get_managed_buffer_storage_mode(),
                    )
                    .unwrap(),
            );
        }

        self.index_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.index_buffer.as_ref().unwrap().length(),
            ));
        self.vertex_position_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.vertex_position_buffer.as_ref().unwrap().length(),
            ));
        self.vertex_normal_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.vertex_normal_buffer.as_ref().unwrap().length(),
            ));
        self.vertex_colour_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.vertex_colour_buffer.as_ref().unwrap().length(),
            ));
        self.per_primitive_data_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.per_primitive_data_buffer.as_ref().unwrap().length(),
            ));

        self.index_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(&NSString::from_str(&format!(
                "index buffer of {}",
                self.name
            ))));
        self.vertex_position_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(&NSString::from_str(&format!(
                "vertex position buffer of {}",
                self.name
            ))));
        self.vertex_normal_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(&NSString::from_str(&format!(
                "vertex normal buffer of {}",
                self.name
            ))));
        self.vertex_colour_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(&NSString::from_str(&format!(
                "vertex colour buffer of {}",
                self.name
            ))));
        self.per_primitive_data_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(&NSString::from_str(&format!(
                "per primitive data buffer of {}",
                self.name
            ))));
    }

    fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
        self.normals.clear();
        self.colours.clear();
        self.triangles.clear();
    }

    fn get_geometry_descriptor(&self) -> Retained<MTLAccelerationStructureGeometryDescriptor> {
        let descriptor = MTLAccelerationStructureTriangleGeometryDescriptor::new();

        descriptor.setIndexBuffer(Some(self.index_buffer.as_ref().unwrap()));
        descriptor.setIndexType(MTLIndexType::UInt16);
        descriptor.setVertexBuffer(Some(self.vertex_position_buffer.as_ref().unwrap()));
        descriptor.setVertexStride(size_of::<Vec4>());
        descriptor.setTriangleCount(self.indices.len() / 3);
        descriptor.setPrimitiveDataBuffer(Some(self.per_primitive_data_buffer.as_ref().unwrap()));
        descriptor.setPrimitiveDataStride(size_of::<Triangle>());
        descriptor.setPrimitiveDataElementSize(size_of::<Triangle>());
        descriptor.into_super()
    }

    fn get_resources(&self) -> Vec<&ProtocolObject<dyn MTLResource>> {
        vec![
            self.index_buffer.as_ref().unwrap().as_ref(),
            self.vertex_normal_buffer.as_ref().unwrap().as_ref(),
            self.vertex_colour_buffer.as_ref().unwrap().as_ref(),
        ]
    }
}

#[repr(C)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

#[repr(C)]
pub struct Sphere {
    pub origin_radius_squared: Vec4,
    pub colour_radius: Vec4,
}

pub struct SphereGeometry {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub sphere_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub bounding_box_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub per_primitive_data_buffer: Option<Retained<ProtocolObject<dyn MTLBuffer>>>,
    pub spheres: Vec<Sphere>,
}

impl SphereGeometry {
    pub fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        Self {
            device: device.retain(),
            sphere_buffer: None,
            bounding_box_buffer: None,
            per_primitive_data_buffer: None,
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere_with_origin(&mut self, origin: Vec3, radius: f32, colour: Vec3) {
        self.spheres.push(Sphere {
            origin_radius_squared: Vec4::from((origin, radius * radius)),
            colour_radius: Vec4::from((colour, radius)),
        });
    }
}

impl Geometry for SphereGeometry {
    fn upload_to_buffers(&mut self) {
        self.sphere_buffer = Some(
            unsafe {
                self.device.newBufferWithBytes_length_options(
                    NonNull::new(self.spheres.as_ptr().cast_mut().cast()).unwrap(),
                    size_of_val(self.spheres.as_slice()),
                    get_managed_buffer_storage_mode(),
                )
            }
            .unwrap(),
        );
        self.sphere_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(ns_string!("sphere buffer")));
        let bounding_boxes = self
            .spheres
            .iter()
            .map(|sphere| BoundingBox {
                min: sphere.origin_radius_squared.xyz() - sphere.colour_radius.w,
                max: sphere.origin_radius_squared.xyz() + sphere.colour_radius.w,
            })
            .collect::<Vec<_>>();
        self.bounding_box_buffer = Some(
            unsafe {
                self.device.newBufferWithBytes_length_options(
                    NonNull::new(bounding_boxes.as_ptr().cast_mut().cast()).unwrap(),
                    size_of_val(bounding_boxes.as_slice()),
                    get_managed_buffer_storage_mode(),
                )
            }
            .unwrap(),
        );
        self.bounding_box_buffer
            .as_ref()
            .unwrap()
            .setLabel(Some(ns_string!("bounding box buffer")));
        self.sphere_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.sphere_buffer.as_ref().unwrap().length(),
            ));
        self.bounding_box_buffer
            .as_ref()
            .unwrap()
            .didModifyRange(NSRange::new(
                0,
                self.bounding_box_buffer.as_ref().unwrap().length(),
            ));
    }

    fn clear(&mut self) {
        self.spheres.clear();
    }

    fn get_geometry_descriptor(&self) -> Retained<MTLAccelerationStructureGeometryDescriptor> {
        let descriptor = MTLAccelerationStructureBoundingBoxGeometryDescriptor::descriptor();
        descriptor.setBoundingBoxBuffer(Some(self.bounding_box_buffer.as_ref().unwrap()));
        descriptor.setBoundingBoxCount(self.spheres.len());
        descriptor.setPrimitiveDataBuffer(Some(self.sphere_buffer.as_ref().unwrap()));
        descriptor.setPrimitiveDataStride(size_of::<Sphere>());
        descriptor.setPrimitiveDataElementSize(size_of::<Sphere>());
        descriptor.into_super()
    }

    fn get_resources(&self) -> Vec<&ProtocolObject<dyn MTLResource>> {
        vec![self.sphere_buffer.as_ref().unwrap().as_ref()]
    }

    fn get_intersection_function_name(&self) -> Option<&NSString> {
        Some(ns_string!("sphereIntersectionFunction"))
    }
}

pub struct GeometryInstance {
    pub geometry: Rc<dyn Geometry>,
    pub transform: Mat4,
    pub mask: u32,
    pub index_in_scene: NSUInteger,
}

#[repr(C)]
pub struct AreaLight {
    pub position: Vec4,
    pub forward: Vec4,
    pub right: Vec4,
    pub up: Vec4,
    pub colour: Vec4,
}
