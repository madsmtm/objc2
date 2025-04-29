use std::ptr::NonNull;
use std::rc::Rc;

use glam::{Mat4, Vec3, Vec4};
use objc2::Message;
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{ns_string, NSRange};
use objc2_metal::{MTLBuffer, MTLDevice, MTLResource};
use rand::Rng;

use super::camera::Camera;
use super::geometry::*;

pub struct Scene {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub camera: Camera,
    pub geometries: Vec<Rc<dyn Geometry>>,
    pub geometry_instances: Vec<GeometryInstance>,
    pub lights: Vec<AreaLight>,
    pub lights_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
}

impl Scene {
    pub fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        let mut geometries = Vec::<Rc<dyn Geometry>>::new();
        let mut light_mesh = TriangleGeometry::new(device, "light".to_string());
        let transform = Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))
            * Mat4::from_scale(Vec3::new(0.5, 1.98, 0.5));
        light_mesh.add_cube_with_faces(
            FACE_MASK_POSITIVE_Y,
            Vec3::new(1.0, 1.0, 1.0),
            transform,
            true,
        );
        light_mesh.upload_to_buffers();
        let light_mesh = Rc::new(light_mesh);
        geometries.push(light_mesh.clone());

        let mut geometry_mesh = TriangleGeometry::new(device, "geometry".to_string());
        let transform = Mat4::from_translation(Vec3::new(0.0, 1.0, 0.0))
            * Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
        geometry_mesh.add_cube_with_faces(
            FACE_MASK_NEGATIVE_Y | FACE_MASK_POSITIVE_Y | FACE_MASK_NEGATIVE_Z,
            Vec3::new(0.725, 0.71, 0.68),
            transform,
            true,
        );
        geometry_mesh.add_cube_with_faces(
            FACE_MASK_NEGATIVE_X,
            Vec3::new(0.63, 0.065, 0.05),
            transform,
            true,
        );
        geometry_mesh.add_cube_with_faces(
            FACE_MASK_POSITIVE_X,
            Vec3::new(0.14, 0.45, 0.091),
            transform,
            true,
        );
        let transform = Mat4::from_translation(Vec3::new(-0.335, 0.6, -0.29))
            * Mat4::from_rotation_y(0.3)
            * Mat4::from_scale(Vec3::new(0.6, 1.2, 0.6));
        geometry_mesh.add_cube_with_faces(
            FACE_MASK_ALL,
            Vec3::new(0.725, 0.71, 0.68),
            transform,
            false,
        );
        geometry_mesh.upload_to_buffers();
        let geometry_mesh = Rc::new(geometry_mesh);
        geometries.push(geometry_mesh.clone());

        let mut sphere_geometry = SphereGeometry::new(device);
        sphere_geometry.add_sphere_with_origin(
            Vec3::new(0.3275, 0.3, 0.3725),
            0.3,
            Vec3::new(0.725, 0.71, 0.68),
        );
        sphere_geometry.upload_to_buffers();
        let sphere_geometry = Rc::new(sphere_geometry);
        geometries.push(sphere_geometry.clone());

        let mut rng = rand::rng();
        let mut geometry_instances = Vec::new();
        let mut lights = Vec::new();
        for y in -1..2 {
            for x in -1..2 {
                let transform =
                    Mat4::from_translation(Vec3::new(x as f32 * 2.5, y as f32 * 2.5, 0.0));
                geometry_instances.push(GeometryInstance {
                    geometry: light_mesh.clone(),
                    transform,
                    mask: GEOMETRY_MASK_LIGHT,
                    index_in_scene: 0,
                });
                geometry_instances.push(GeometryInstance {
                    geometry: geometry_mesh.clone(),
                    transform,
                    mask: GEOMETRY_MASK_TRIANGLE,
                    index_in_scene: 1,
                });
                geometry_instances.push(GeometryInstance {
                    geometry: sphere_geometry.clone(),
                    transform,
                    mask: GEOMETRY_MASK_SPHERE,
                    index_in_scene: 2,
                });
                lights.push(AreaLight {
                    position: Vec4::new(x as f32 * 2.5, y as f32 * 2.5 + 1.98, 0.0, 0.0),
                    forward: Vec4::new(0.0, -1.0, 0.0, 0.0),
                    right: Vec4::new(0.25, 0.0, 0.0, 0.0),
                    up: Vec4::new(0.0, 0.0, 0.25, 0.0),
                    colour: Vec4::new(
                        rng.random_range(0f32..=1.0),
                        rng.random_range(0f32..=1.0),
                        rng.random_range(0f32..=1.0),
                        0.0,
                    ),
                });
            }
        }
        let lights_buffer = unsafe {
            device.newBufferWithBytes_length_options(
                NonNull::new(lights.as_ptr().cast_mut().cast()).unwrap(),
                size_of_val(lights.as_slice()),
                get_managed_buffer_storage_mode(),
            )
        }
        .unwrap();
        lights_buffer.didModifyRange(NSRange::new(0, lights_buffer.length()));
        lights_buffer.setLabel(Some(ns_string!("lights buffer")));

        Self {
            device: device.retain(),
            camera: Camera::new(),
            geometries,
            geometry_instances,
            lights,
            lights_buffer,
        }
    }
}
