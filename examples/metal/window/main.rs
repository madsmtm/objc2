// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate cocoa;
extern crate metal;
extern crate winit;
#[macro_use]
extern crate objc;
extern crate objc_id;
extern crate objc_foundation;

use cocoa::base::id as cocoa_id;
use cocoa::base::{selector, class};
use cocoa::foundation::{NSUInteger, NSRect, NSPoint, NSSize,
                        NSAutoreleasePool, NSProcessInfo};
use cocoa::appkit::{NSApp,
                    NSApplication, NSApplicationActivationPolicyRegular,
                    NSWindow, NSTitledWindowMask, NSBackingStoreBuffered,
                    NSMenu, NSMenuItem, NSRunningApplication, NSView,
                    NSApplicationActivateIgnoringOtherApps};

use objc::Message;
use objc::runtime::{Object, Class, BOOL, YES, NO};
use objc_id::{Id, ShareId};
use objc_foundation::{INSObject, NSString, INSString};

use metal::*;

use winit::os::macos::WindowExt;

use std::ffi::CStr;
use std::mem;
use std::marker::PhantomData;

fn prepare_pipeline_state<'a>(device: MTLDevice, library: MTLLibrary) -> MTLRenderPipelineState {
    let vert = library.get_function("triangle_vertex").unwrap();
    let frag = library.get_function("triangle_fragment").unwrap();

    let pipeline_state_descriptor = MTLRenderPipelineDescriptor::alloc().init();
    pipeline_state_descriptor.set_vertex_function(vert);
    pipeline_state_descriptor.set_fragment_function(frag);
    pipeline_state_descriptor.color_attachments().object_at(0).set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    let pipeline_state = device.new_render_pipeline_state(pipeline_state_descriptor).unwrap();

    pipeline_state
}

fn prepare_render_pass_descriptor(descriptor: MTLRenderPassDescriptor, texture: MTLTexture) {
    let color_attachment = descriptor.color_attachments().object_at(0);

    color_attachment.set_texture(texture);
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.2, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn main() {
    let glutin_window = winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Metal".into()).build().unwrap();

    unsafe {
        let window: cocoa_id = mem::transmute(glutin_window.get_nswindow());
        let device = create_system_default_device();

        let layer = CAMetalLayer::layer();
        layer.set_device(device);
        layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

        let view = window.contentView();
        view.setWantsBestResolutionOpenGLSurface_(YES);
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.0));

        let draw_size = glutin_window.get_inner_size().unwrap();
        layer.set_drawable_size(NSSize::new(draw_size.0 as f64, draw_size.1 as f64));

        let library = device.new_default_library();
        let pipeline_state = prepare_pipeline_state(device, library);
        let render_pass_descriptor = MTLRenderPassDescriptor::alloc().init();
        let command_queue = device.new_command_queue();

        let vbuf = {
            let vertex_data = [
                  0.0f32,  0.5, 1.0, 0.0, 0.0,
                 -0.5, -0.5, 0.0, 1.0, 0.0,
                  0.5,  0.5, 0.0, 0.0, 1.0,
            ];

            device.new_buffer_with_data(
                mem::transmute(vertex_data.as_ptr()),
                (vertex_data.len() * mem::size_of::<f32>()) as u64,
                MTLResourceOptionCPUCacheModeDefault)
        };

        loop {
            for event in glutin_window.poll_events() {
                match event {
                    winit::Event::Closed => break,
                    _ => ()
                }
            }

            if let Some(drawable) = layer.next_drawable() {
                prepare_render_pass_descriptor(render_pass_descriptor, drawable.texture());

                let command_buffer = command_queue.new_command_buffer();
                let encoder = command_buffer.new_render_command_encoder(render_pass_descriptor);
                encoder.set_render_pipeline_state(pipeline_state);
                encoder.set_vertex_buffer(0, 0, vbuf);
                encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
                encoder.end_encoding();

                command_buffer.present_drawable(drawable);
                command_buffer.commit();

                encoder.release();
                command_buffer.release();
                drawable.release();
            }
        }
    }
}
