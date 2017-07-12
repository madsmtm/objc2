// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate cocoa;
extern crate metal_rs as metal;
extern crate winit;
extern crate objc;
extern crate objc_foundation;
extern crate block;
extern crate sema;

use cocoa::base::id as cocoa_id;
use cocoa::foundation::{NSRange, NSSize};
use cocoa::appkit::{NSWindow, NSView};

use objc::runtime::YES;

use metal::*;

use winit::os::macos::WindowExt;

use std::mem;


fn prepare_pipeline_state<'a>(device: MTLDevice, library: MTLLibrary) -> MTLRenderPipelineState {
    let vert = library.get_function("triangle_vertex");
    let frag = library.get_function("triangle_fragment");

    let pipeline_state_descriptor = MTLRenderPipelineDescriptor::alloc().init();
    pipeline_state_descriptor.set_vertex_function(vert);
    pipeline_state_descriptor.set_fragment_function(frag);
    pipeline_state_descriptor.color_attachments().object_at(0).set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    let pipeline_state = device.new_render_pipeline_state(pipeline_state_descriptor).unwrap();

    pipeline_state
}

fn prepare_render_pass_descriptor(descriptor: MTLRenderPassDescriptor, texture: MTLTexture) {
    //descriptor.color_attachments().set_object_at(0, MTLRenderPassColorAttachmentDescriptor::alloc());
    //let color_attachment: MTLRenderPassColorAttachmentDescriptor = unsafe { msg_send![descriptor.color_attachments().0, _descriptorAtIndex:0] };//descriptor.color_attachments().object_at(0);
    let color_attachment = descriptor.color_attachments().object_at(0);

    color_attachment.set_texture(texture);
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.5, 0.2, 0.2, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let glutin_window = winit::WindowBuilder::new()
        .with_dimensions(800, 600)
        .with_title("Metal".to_string())
        .build(&events_loop).unwrap();

    let window: cocoa_id = unsafe { mem::transmute(glutin_window.get_nswindow()) };
    let device = create_system_default_device();

    let layer = CAMetalLayer::layer();
    layer.set_device(device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    unsafe {
        let view = window.contentView();
        view.setWantsBestResolutionOpenGLSurface_(YES);
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.0));
    }

    let draw_size = glutin_window.get_inner_size().unwrap();
    layer.set_drawable_size(NSSize::new(draw_size.0 as f64, draw_size.1 as f64));

    let library = device.new_library_with_file("examples/window/default.metallib").unwrap();
    let pipeline_state = prepare_pipeline_state(device, library);
    let command_queue = device.new_command_queue();
    //let nc: () = msg_send![command_queue.0, setExecutionEnabled:true];

    let vbuf = {
        let vertex_data = [
              0.0f32,  0.5, 1.0, 0.0, 0.0,
             -0.5, -0.5, 0.0, 1.0, 0.0,
              0.5,  0.5, 0.0, 0.0, 1.0,
        ];

        device.new_buffer_with_data(
            unsafe { mem::transmute(vertex_data.as_ptr()) },
            (vertex_data.len() * mem::size_of::<f32>()) as u64,
            MTLResourceOptionCPUCacheModeDefault)
    };

    let mut pool = NSAutoreleasePool::alloc().init();
    let mut r = 0.0f32;
    let mut running = true;

    while running {
        events_loop.poll_events(|event| {
            match event {
                winit::Event::WindowEvent{ event: winit::WindowEvent::Closed, .. } => running = false,
                _ => ()
            }
        });

        if let Some(drawable) = layer.next_drawable() {
            let render_pass_descriptor = MTLRenderPassDescriptor::new();
            let _a = prepare_render_pass_descriptor(render_pass_descriptor, drawable.texture());

            let command_buffer = command_queue.new_command_buffer();
            let parallel_encoder = command_buffer.new_parallel_render_command_encoder(render_pass_descriptor);
            let encoder = parallel_encoder.render_command_encoder();
            encoder.set_render_pipeline_state(pipeline_state);
            encoder.set_vertex_buffer(0, 0, vbuf);
            encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
            encoder.end_encoding();
            parallel_encoder.end_encoding();

            render_pass_descriptor.color_attachments().object_at(0).set_load_action(MTLLoadAction::DontCare);

            let parallel_encoder = command_buffer.new_parallel_render_command_encoder(render_pass_descriptor);
            let encoder = parallel_encoder.render_command_encoder();
            use std::mem;
            let p = vbuf.contents();
            let vertex_data: &[u8; 60] = unsafe { mem::transmute(&[
                  0.0f32,  0.5, 1.0, 0.0-r, 0.0,
                 -0.5, -0.5, 0.0, 1.0-r, 0.0,
                  0.5,  0.5, 0.0, 0.0, 1.0+r,
            ]) };
            use std::ptr;

            unsafe {
                ptr::copy(vertex_data.as_ptr(), p as *mut u8, (vertex_data.len() * mem::size_of::<f32>()) as usize);
            }
            vbuf.did_modify_range(NSRange::new(0 as u64, (vertex_data.len() * mem::size_of::<f32>()) as u64));


            encoder.set_render_pipeline_state(pipeline_state);
            encoder.set_vertex_buffer(0, 0, vbuf);
            encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
            encoder.end_encoding();
            parallel_encoder.end_encoding();

            command_buffer.present_drawable(drawable);
            command_buffer.commit();

            r += 0.01f32;
            //let _: () = msg_send![command_queue.0, _submitAvailableCommandBuffers];

            unsafe { pool.release() };
            pool = NSAutoreleasePool::alloc().init();
        }
    }
}
